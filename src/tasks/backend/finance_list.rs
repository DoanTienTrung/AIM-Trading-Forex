#![allow(clippy::type_complexity)]
use crate::slint_generatedAppWindow::{FinanceList, FinanceName, FinanceValue};
use aim_data::aim::FinanceSheetData;
use slint::{ModelRc, SharedString};
// Extension trait to add the from_data function to FinanceList
pub trait FinanceListExt {
    fn from_data(balance_data_vec: Vec<Vec<FinanceSheetData>>) -> Self;
}

// Create a thread-safe wrapper around FinanceList
// This allows us to build the FinanceList in a thread-safe way
pub struct ThreadSafeFinanceList {
    names: Vec<FinanceName>,
    values: Vec<FinanceValue>,
    expanded: Vec<bool>,
}

impl ThreadSafeFinanceList {
    pub fn new() -> Self {
        Self {
            names: Vec::new(),
            values: Vec::new(),
            expanded: Vec::new(),
        }
    }

    pub fn into_finance_list(self) -> FinanceList {
        FinanceList {
            name: ModelRc::new(slint::VecModel::from(self.names)),
            value: ModelRc::new(slint::VecModel::from(self.values)),
            expanded: ModelRc::new(slint::VecModel::from(self.expanded)),
        }
    }
}

impl FinanceListExt for FinanceList {
    /// Convert a vector of FinanceSheetData into a FinanceList structure
    ///
    /// This function groups balance sheet data by financial item and organizes it
    /// into a hierarchical structure suitable for display in the UI.
    fn from_data(balance_data_vec: Vec<Vec<FinanceSheetData>>) -> Self {
        let mut thread_safe_list = ThreadSafeFinanceList::new();

        // If we have no data, return an empty FinanceList
        if balance_data_vec.is_empty() {
            return thread_safe_list.into_finance_list();
        }

        // Step 1: Collect all financial items from all periods
        let mut items_by_id = std::collections::HashMap::new();
        let mut period_names = Vec::new();

        // First pass: collect all items across all periods
        for period_data in &balance_data_vec {
            if period_data.is_empty() {
                continue;
            }

            // Get the period name
            if let Some(item) = period_data.first() {
                if let Some(period) = &item.period {
                    if !period_names.contains(period) {
                        period_names.push(period.clone());
                    }
                }
            }

            // Collect all items in this period
            for item in period_data {
                // Only process items that have a name
                if let Some(name) = &item.name {
                    // Keep original level values from server (DO NOT ADJUST THEM)
                    let level = item.level.unwrap_or(0); // Default to 0 if level is not provided
                    let parent_id = item.parent_id.unwrap_or(-1);

                    // Force all items to be expanded regardless of server data
                    items_by_id.insert(
                        item.id,
                        (
                            name.clone(),
                            parent_id,
                            level,
                            true, // Force expanded to true for all items
                        ),
                    );
                }
            }
        }

        // Step 2: Build a tree structure for proper hierarchical sorting
        // First, group items by their level
        let mut items_by_level: std::collections::HashMap<i32, Vec<(i64, String, i64, i32, bool)>> =
            std::collections::HashMap::new();

        // Group items by level
        for (id, (name, parent_id, level, expanded)) in &items_by_id {
            items_by_level.entry(*level).or_default().push((
                *id,
                name.clone(),
                *parent_id,
                *level,
                *expanded,
            ));
        }

        // Get all available levels and sort them
        let mut levels: Vec<i32> = items_by_level.keys().cloned().collect();
        levels.sort();

        // Now we'll create a sorted list that preserves the hierarchy
        let mut sorted_items = Vec::new();

        // First, build a mapping of parent_id to child ids
        let mut items_by_parent: std::collections::HashMap<i64, Vec<i64>> =
            std::collections::HashMap::new();
        for (id, (_, parent_id, _, _)) in &items_by_id {
            items_by_parent.entry(*parent_id).or_default().push(*id);
        }

        // Sort children within each parent group by ID for consistent ordering
        for (_, children) in items_by_parent.iter_mut() {
            children.sort_unstable();
        }

        // Function to recursively add items and their children to the sorted list
        fn add_with_children(
            id: i64,
            items_by_id: &std::collections::HashMap<i64, (String, i64, i32, bool)>,
            items_by_parent: &std::collections::HashMap<i64, Vec<i64>>,
            sorted_items: &mut Vec<(i64, String, i64, i32, bool)>,
        ) {
            if let Some(&(ref name, parent_id, level, expanded)) = items_by_id.get(&id) {
                sorted_items.push((id, name.clone(), parent_id, level, expanded));

                // Add all children in order
                if let Some(children) = items_by_parent.get(&id) {
                    for child_id in children {
                        add_with_children(*child_id, items_by_id, items_by_parent, sorted_items);
                    }
                } else {
                    log::error!("No children found for parent id={id}, name={name}");
                }
            } else {
                log::error!("Could not find item with id={id} in items_by_id map");
            }
        }

        // Start with top-level items (parent_id = -1) and recursively add their children
        if let Some(top_level_items) = items_by_parent.get(&-1) {
            for id in top_level_items {
                add_with_children(*id, &items_by_id, &items_by_parent, &mut sorted_items);
            }
        } else {
            log::error!("No top-level items found with parent_id=-1");
        }

        // Build a mapping from ID to position in the sorted list for quick lookups
        let id_to_position: std::collections::HashMap<i64, usize> = sorted_items
            .iter()
            .enumerate()
            .map(|(index, (id, _, _, _, _))| (*id, index))
            .collect();

        // Step 3: Create the FinanceName entries from our sorted items
        for (id, name, parent_id, level, _) in sorted_items.iter() {
            // Find the parent's position in the vector (if it exists)
            let parent_idx = if *parent_id == -1 {
                // Top-level items keep parent_id = -1
                -1
            } else {
                // For other items, find the index of their parent in the vector
                match id_to_position.get(parent_id) {
                    Some(&position) => position as i32,
                    None => {
                        log::error!("Could not find parent position for id={id}, using -1");
                        -1 // Default to top-level if parent not found
                    }
                }
            };

            thread_safe_list.names.push(FinanceName {
                name: SharedString::from(name.clone()),
                // Adjust level by subtracting 1 (to match UI expectations)
                level: *level - 1,
                // Use the index of the parent in the vector, not the ID from server
                parent_id: parent_idx,
            });

            // Override all expanded states to true to ensure all items are visible
            thread_safe_list.expanded.push(true);
        }

        // Step 3: Process values for each period
        for period_data in &balance_data_vec {
            if period_data.is_empty() {
                continue;
            }

            // Extract the period from the first item
            let period_name = if let Some(first_item) = period_data.first() {
                if let Some(period) = &first_item.period {
                    period.clone()
                } else {
                    continue; // Skip if no period information
                }
            } else {
                continue; // Skip if there's no first item
            };

            // Create a value entry for this period with all items, initialize with 0.0
            let mut period_values = vec![0.0; sorted_items.len()];

            for item in period_data {
                if let Some(&position) = id_to_position.get(&item.id) {
                    // Only set the value if it's Some, otherwise leave it as 0.0
                    if let Some(value) = item.value {
                        period_values[position] = value as f32;
                    }
                }
            }

            thread_safe_list.values.push(FinanceValue {
                quarter: SharedString::from(period_name),
                items: ModelRc::new(slint::VecModel::from(period_values)),
            });
        }

        // Convert the thread-safe list to a FinanceList
        thread_safe_list.into_finance_list()
    }
}
