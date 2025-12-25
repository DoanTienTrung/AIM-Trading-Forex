# Corporate Profile API Documentation

## Get Institution Profile

**Endpoint:**
```
GET http://103.48.84.52:4040/institution-profile/{symbol}
```

**Parameters:**
- `{symbol}`: The stock symbol of the institution (e.g., AAA).

**Description:**
This endpoint returns detailed profile information for a given institution symbol.

**Response Structure:**
The response is a JSON object with the following fields:
```json
{
  "institutionID": 703,
  "symbol": "AAA",
  "icbCode": "1353",
  "companyName": "CTCP Nhựa An Phát Xanh",
  "shortName": "An Phat BioPlastic., JSC",
  "internationalName": "An Phat Plastic and Green Environment Joint Stock Company",
  "headQuarters": "Lô CN11 + CN12, Cụm Công nghiệp An Đồng, Huyện Nam Sách, Tỉnh Hải Dương",
  "phone": "+84 (320) 375-5998",
  "fax": "+84 (320) 375-5113",
  "email": "anphat@anphatplastic.com",
  "webAddress": "www.anphatbioplastics.com",
  "overview": "Công ty Cổ phần DAP-VINACHEM là một trong những doanh nghiệp sản xuất phân bón hàng đầu tại Việt Nam...",
  "history": "<ul>...</ul>",
  "businessAreas": "<ul>...</ul>",
  "employees": 2861,
  "branches": null,
  "establishmentDate": "2007-03-09T00:00:00",
  "businessLicenseNumber": "0800373586",
  "dateOfIssue": "2023-04-10T00:00:00",
  "taxIDNumber": "0800373586",
  "charterCapital": 3822744960000.0,
  "dateOfListing": "2016-11-25T00:00:00",
  "exchange": "HSX",
  "initialListingPrice": 31200.0,
  "listingVolume": 382274496.0,
  "stateOwnership": 0.0,
  "foreignOwnership": 0.021758017045427,
  "otherOwnership": 0.978241982954573,
  "isListed": true
}
```

**Rust Struct:**
```rust
    pub institution_id: i64,
    pub symbol: String,
    pub icb_code: Option<String>,
    pub company_name: Option<String>,
    pub short_name: Option<String>,
    pub international_name: Option<String>,
    pub head_quarters: Option<String>,
    pub phone: Option<String>,
    pub fax: Option<String>,
    pub email: Option<String>,
    pub web_address: Option<String>,
    pub overview: Option<String>,
    pub history: Option<String>,
    pub business_areas: Option<String>,
    pub employees: Option<i32>,
    pub branches: Option<String>,
    pub establishment_date: Option<chrono::NaiveDateTime>,
    pub business_license_number: Option<String>,
    pub date_of_issue: Option<chrono::NaiveDateTime>,
    pub tax_id_number: Option<String>,
    pub charter_capital: Option<f64>,
    pub date_of_listing: Option<chrono::NaiveDateTime>,
    pub exchange: Option<String>,
    pub initial_listing_price: Option<f64>,
    pub listing_volume: Option<f64>,
    pub state_ownership: Option<f64>,
    pub foreign_ownership: Option<f64>,
    pub other_ownership: Option<f64>,
    pub is_listed: Option<bool>,
}
```

**Notes for Developers:**
- All fields are returned as JSON. Optional fields may be null.
- Dates are in ISO 8601 format (e.g., "2007-03-09T00:00:00").
- Ownership fields and capital are floating-point numbers.
- Use the symbol parameter to query different institutions.

**Error Handling:**
If the symbol does not exist, the API will return a 404 or an empty object.

---

## Get Shareholder List

**Endpoint:**
```
GET http://103.48.84.52:4040/shareholder/{ticker}
```

**Parameters:**
- `{ticker}`: The stock symbol to query shareholders for (e.g., AAA).

**Description:**
This endpoint returns a list of shareholders for the specified stock symbol.

**Response Structure:**
The response is a JSON array of objects, each with the following fields:

```json

[
  {
    "id": 1,
    "ticker": "AAA",
    "majorholderid": 123,
    "individualholderid": 456,
    "institutionholderid": 789,
    "institutionholdersymbol": "XYZ",
    "institutionholderexchange": "HSX",
    "name": "Nguyen Van A",
    "position": "Director",
    "shares": 100000.0,
    "ownership": 5.5,
    "isorganization": false,
    "isforeigner": false,
    "isfounder": true,
    "reported_at": 1710000000
  }
]
```

**Rust Struct:**
```rust
    pub id: i32,
    pub ticker: String,
    pub majorholderid: Option<i32>,
    pub individualholderid: Option<i32>,
    pub institutionholderid: Option<i32>,
    pub institutionholdersymbol: Option<String>,
    pub institutionholderexchange: Option<String>,
    pub name: String,
    pub position: Option<String>,
    pub shares: Option<f32>,
    pub ownership: Option<f32>,
    pub isorganization: Option<bool>,
    pub isforeigner: Option<bool>,
    pub isfounder: Option<bool>,
    pub reported_at: Option<i64>,
}
```

**Field Notes:**
- All fields are returned as JSON. Optional fields may be null.
- `shares` and `ownership` are floating-point numbers.
- `reported_at` is a Unix timestamp (seconds since epoch).
- Use the ticker parameter to query different stocks.

**Error Handling:**
If the ticker does not exist or has no shareholders, the API will return an empty array.

---

## Get Insider Transactions

**Endpoint:**
```
GET http://103.48.84.52:4040/insider-transactions/{symbol}
```

**Parameters:**
- `{symbol}`: The stock symbol to query insider transactions for (e.g., AAA).

**Description:**
This endpoint returns a list of insider transactions for the specified stock symbol.

**Response Structure:**
The response is a JSON array of objects, each with the following fields:

```json
[
  {
    "transaction_id": 123456,
    "major_holder_id": 789,
    "individual_holder_id": 456,
    "institution_holder_id": 321,
    "institution_holder_symbol": "XYZ",
    "institution_holder_exchange": "HSX",
    "name": "Nguyen Van B",
    "position": "Director",
    "symbol": "AAA",
    "type": 1,
    "execution_volume": 10000.0,
    "execution_date": 1710000000,
    "start_date": 1709000000,
    "end_date": 1711000000,
    "registered_volume": 12000.0
  }
]
```

**Rust Struct:**
```rust
    pub transaction_id: i64,
    pub major_holder_id: Option<i32>,
    pub individual_holder_id: Option<i32>,
    pub institution_holder_id: Option<i32>,
    pub institution_holder_symbol: Option<String>,
    pub institution_holder_exchange: Option<String>,
    pub name: String,
    pub position: Option<String>,
    pub symbol: String,
    pub r#type: Option<i32>, // 1: Sell, 0: Buy
    pub execution_volume: Option<f32>,
    pub execution_date: Option<i64>,
    pub start_date: Option<i64>,
    pub end_date: Option<i64>,
    pub registered_volume: Option<f32>,
}
```

**Field Notes:**
- All fields are returned as JSON. Optional fields may be null.
- `type`: 1 = Sell, 0 = Buy.
- `execution_volume` and `registered_volume` are floating-point numbers.
- `execution_date`, `start_date`, and `end_date` are Unix timestamps (seconds since epoch).
- Use the symbol parameter to query different stocks.

**Error Handling:**
If the symbol does not exist or has no insider transactions, the API will return an empty array.

---

## Get Subsidiaries

**Endpoint:**
```
GET http://103.48.84.52:4040/subsidiaries/{symbol}
```

**Parameters:**
- `{symbol}`: The stock symbol to query subsidiaries for (e.g., AAA).

**Description:**
This endpoint returns a list of subsidiaries and associates for the specified institution symbol.

**Response Structure:**
The response is a JSON array of objects, each with the following fields:

```json
[
  {
    "institution_id": 1234,
    "father_symbol": "AAA",
    "symbol": "BBB",
    "exchange": "HSX",
    "company_name": "Subsidiary Company Ltd.",
    "short_name": "SubCo",
    "international_name": "Subsidiary Company Limited",
    "company_profile": "Profile text...",
    "type": 0,
    "ownership": 51.0,
    "shares": 1000000.0,
    "is_listed": true,
    "charter_capital": 500000000.0
  }
]
```

**Rust Struct:**
```rust
    pub institution_id: i32,
    pub father_symbol: Option<String>,
    pub symbol: Option<String>,
    pub exchange: Option<String>,
    pub company_name: Option<String>,
    pub short_name: Option<String>,
    pub international_name: Option<String>,
    pub company_profile: Option<String>,
    pub r#type: Option<i32>, // 0: Subsidiary, 1: Associate
    pub ownership: Option<f64>,
    pub shares: Option<f64>,
    pub is_listed: Option<bool>,
    pub charter_capital: Option<f64>,
}
```

**Field Notes:**
- All fields are returned as JSON. Optional fields may be null.
- `type`: 0 = Subsidiary, 1 = Associate.
- `ownership`, `shares`, and `charter_capital` are floating-point numbers.
- Use the symbol parameter to query different institutions.

**Error Handling:**
If the symbol does not exist or has no subsidiaries, the API will return an empty array.

---

## Get Corporate Events

**Endpoint:**
```
GET http://103.48.84.52:4040/event/{symbol}
```

**Parameters:**
- `{symbol}`: The stock symbol to query corporate events for (e.g., AAA).

**Description:**
This endpoint returns a list of corporate events (such as dividends, stock splits, rights issues, etc.) for the specified stock symbol.

**Response Structure:**
The response is a JSON array of objects, each with the following fields:

```json
[
    {
        "eventID": 59967,
        "symbol": "AAA",
        "name": "CTCP Nhựa An Phát Xanh",
        "title": "Cổ tức đợt 1/2024 bằng tiền, tỷ lệ 300đ/CP",
        "recordDate": "2025-05-30T00:00:00",
        "registrationDate": "2025-06-02T00:00:00",
        "executionDate": "2025-06-20T00:00:00",
        "type": 1
    },
    {
        "eventID": 46562,
        "symbol": "AAA",
        "name": "CTCP Nhựa An Phát Xanh",
        "title": "Cổ tức năm 2021 bằng cổ phiếu, tỷ lệ 10:1",
        "recordDate": "2021-09-01T00:00:00",
        "registrationDate": "2021-09-06T00:00:00",
        "executionDate": "2021-10-12T00:00:00",
        "type": 2
    }
]
```

**Rust Struct:**
```rust
pub struct CorporateEvent {
    pub event_id: i64,
    pub symbol: String,
    pub name: Option<String>,
    pub title: Option<String>,
    pub record_date: Option<i64>,
    pub registration_date: Option<i64>,
    pub execution_date: Option<i64>,
    pub r#type: Option<i32>, // 1: Bang tien, 2: Bang CP, 3: Phát hành CP cho CĐHH
}
```

**Error Handling:**
If the symbol does not exist or has no corporate events, the API will return an empty array.

---

## Get Officers

**Endpoint:**
```
GET http://103.48.84.52:4040/officer/{symbol}
```

**Parameters:**
- `{symbol}`: The stock symbol to query officers for (e.g., AAA).

**Description:**
This endpoint returns a list of officers for the specified stock symbol.

**Response Structure:**
The response is a JSON array of objects, each with the following fields:

```json
[
  {
    "officer_id": 123,
    "symbol": "AAA",
    "individual_id": 456,
    "name": "Nguyen Van C",
    "position_id": 1,
    "position": "CEO",
    "is_foreigner": false
  }
]
```

**Rust Struct:**
```rust
pub struct Officer {
    pub officer_id: i32,
    pub symbol: String,
    pub individual_id: Option<i32>,
    pub name: String,
    pub position_id: Option<i32>,
    pub position: Option<String>,
    pub is_foreigner: Option<bool>,
}
```

**Error Handling:**
If the symbol does not exist or has no officers, the API will return an empty array.

---

## Get Individual Profile

**Endpoint:**
```
GET http://103.48.84.52:4040/individual_profile/{individual_id}
```

**Parameters:**
- `{individual_id}`: The unique ID of the individual to query (e.g., 15809).

**Description:**
This endpoint returns detailed profile information for a given individual by their ID.

**Response Structure:**
The response is a JSON object with the following fields:

```json
{
  "individual_id": 15809,
  "is_foreign": false,
  "name": "Nguyen Van D",
  "bio": "CEO of Example Corp.",
  "gender": 1,
  "has_photo": true,
  "photo_url": "http://example.com/photo.jpg",
  "date_of_birth": "1970-01-01",
  "home_town": "Hanoi",
  "education": 3,
  "place_of_birth": "Hanoi",
  "is_dead": false,
  "symbol": "AAA",
  "position_name": "CEO",
  "institution_name": "Example Corp.",
  "institution_is_listing": true,
  "institution_symbol": "AAA",
  "asset": 1000000000.0
}
```

**Rust Struct:**
```rust
pub struct IndividualProfile {
    pub individual_id: i64,
    pub is_foreign: Option<bool>,
    pub name: Option<String>,
    pub bio: Option<String>,
    pub gender: Option<i32>,
    pub has_photo: Option<bool>,
    pub photo_url: Option<String>,
    pub date_of_birth: Option<String>,
    pub home_town: Option<String>,
    pub education: Option<i32>,
    pub place_of_birth: Option<String>,
    pub is_dead: Option<bool>,
    pub symbol: Option<String>,
    pub position_name: Option<String>,
    pub institution_name: Option<String>,
    pub institution_is_listing: Option<bool>,
    pub institution_symbol: Option<String>,
    pub asset: Option<f64>,
}
```

**Error Handling:**
If the individual does not exist, the API will return a 404 or an empty object.

---
## GET INDIVIDIAL PROFILE IMAGE
**Endpoint:**
```
GET http://103.48.84.52:4040/profile_image/{individual_id}
```
**Description**
Returns the profile image (JPEG) for the specified individual.
If the image does not exist, returns a JSON error message.

**Notes**
The image is served directly as a JPEG file for display or download.
If the image file /home/images/{individual_id}.jpg does not exist, you will get a JSON error.
For errors, the response is always JSON with an appropriate HTTP status code.

---
## Get Balance Sheet Items

**Endpoint:**
```
GET http://103.48.84.52:4040/balance-sheet/{symbol}/{period}
```

**Parameters:**
- `{symbol}`: The stock symbol to query (e.g., AAA)
- `{period}`: The period to query (e.g., Q12025 for Quarter 1, 2025)

**Description:**
This endpoint returns a list of balance sheet items for the specified stock symbol and period.

**Response Structure:**
The response is a JSON array of objects, each with the following fields:

```json

[
  {
    "id": 1,
    "name": "TÀI SẢN",
    "parent_id": -1,
    "expanded": true,
    "level": 1,
    "field": null,
    "period": "Q12025",
    "year": 2025,
    "quarter": 1,
    "value": null,
    "symbol": "AAA"
  },
  {
    "id": 101,
    "name": "A. Tài sản lưu động và đầu tư ngắn hạn",
    "parent_id": 1,
    "expanded": true,
    "level": 2,
    "field": null,
    "period": "Q12025",
    "year": 2025,
    "quarter": 1,
    "value": 2097301823647.0,
    "symbol": "AAA"
  }
]
```

**Rust Struct:**
```rust
pub struct BalanceSheetItem {
    pub id: i64,
    pub name: Option<String>,
    pub parent_id: Option<i64>,
    pub expanded: Option<bool>,
    pub level: Option<i32>,
    pub field: Option<String>,
    pub period: Option<String>,
    pub year: Option<i32>,
    pub quarter: Option<i32>,
    pub value: Option<f64>,
    pub symbol: Option<String>,
}
```

**Error Handling:**
If the symbol or period does not exist or has no balance sheet items, the API will return an empty array.

---

## Get Income Statement Items

**Endpoint:**
```
GET http://103.48.84.52:4040/income-statement/{symbol}/{period}
```

**Parameters:**
- `{symbol}`: The stock symbol to query (e.g., AAA)
- `{period}`: The period to query (e.g., Q12025 for Quarter 1, 2025)

**Description:**
This endpoint returns a list of income statement items for the specified stock symbol and period.

**Response Structure:**
The response is a JSON array of objects, each with the following fields:

```json
[
  {
    "id": 1,
    "name": "Doanh thu bán hàng và cung cấp dịch vụ",
    "parent_id": null,
    "expanded": true,
    "level": 1,
    "field": null,
    "period": "Q12025",
    "year": 2025,
    "quarter": 1,
    "value": 123456789.0,
    "symbol": "AAA"
  }
]
```

**Rust Struct:**
```rust
pub struct IncomeStatementItem {
    pub id: i64,
    pub name: Option<String>,
    pub parent_id: Option<i64>,
    pub expanded: Option<bool>,
    pub level: Option<i32>,
    pub field: Option<String>,
    pub period: Option<String>,
    pub year: Option<i32>,
    pub quarter: Option<i32>,
    pub value: Option<f64>,
    pub symbol: Option<String>,
}
```

**Error Handling:**
If the symbol or period does not exist or has no income statement items, the API will return an empty array.

---

## Get Cash Flow Indirect Items

**Endpoint:**
```
GET http://103.48.84.52:4040/cash-flow-indirect/{symbol}/{period}
```

**Parameters:**
- `{symbol}`: The stock symbol to query (e.g., AAA)
- `{period}`: The period to query (e.g., Q12025 for Quarter 1, 2025)

**Description:**
This endpoint returns a list of indirect cash flow items for the specified stock symbol and period.

**Response Structure:**
The response is a JSON array of objects, each with the following fields:

```json
[
  {
    "id": 1,
    "name": "Lưu chuyển tiền từ hoạt động kinh doanh",
    "parent_id": null,
    "expanded": true,
    "level": 1,
    "field": null,
    "period": "Q12025",
    "year": 2025,
    "quarter": 1,
    "value": 123456789.0,
    "symbol": "AAA"
  }
]
```

**Rust Struct:**
```rust
pub struct CashFlowIndirectItem {
    pub id: i64,
    pub name: Option<String>,
    pub parent_id: Option<i64>,
    pub expanded: Option<bool>,
    pub level: Option<i32>,
    pub field: Option<String>,
    pub period: Option<String>,
    pub year: Option<i32>,
    pub quarter: Option<i32>,
    pub value: Option<f64>,
    pub symbol: Option<String>,
}
```

**Error Handling:**
If the symbol or period does not exist or has no cash flow indirect items, the API will return an empty array.

---

## Get Cash Flow Direct Items

**Endpoint:**
```
GET http://103.48.84.52:4040/cash-flow-direct/{symbol}/{period}
```

**Parameters:**
- `{symbol}`: The stock symbol to query (e.g., AAA)
- `{period}`: The period to query (e.g., Q12025 for Quarter 1, 2025)

**Description:**
This endpoint returns a list of direct cash flow items for the specified stock symbol and period.

**Response Structure:**
The response is a JSON array of objects, each with the following fields:

```json
[
  {
    "id": 1,
    "name": "Lưu chuyển tiền từ hoạt động kinh doanh",
    "parent_id": null,
    "expanded": true,
    "level": 1,
    "field": null,
    "period": "Q12025",
    "year": 2025,
    "quarter": 1,
    "value": 123456789.0,
    "symbol": "AAA"
  }
]
```

**Rust Struct:**
```rust
pub struct CashFlowDirectItem {
    pub id: i64,
    pub name: Option<String>,
    pub parent_id: Option<i64>,
    pub expanded: Option<bool>,
    pub level: Option<i32>,
    pub field: Option<String>,
    pub period: Option<String>,
    pub year: Option<i32>,
    pub quarter: Option<i32>,
    pub value: Option<f64>,
    pub symbol: Option<String>,
}
```

**Error Handling:**
If the symbol or period does not exist or has no cash flow direct items, the API will return an empty array.

---


## Get Stock Info

**Endpoint:**
```
GET http://103.48.84.52:4040/stock-info/{ticker}
```

**Parameters:**
- `{ticker}`: The stock symbol to query (e.g., AAA)

**Description:**
This endpoint returns detailed information about a stock, including company details, listing info, and sector classification.

**Response Structure:**
The response is a JSON object with the following fields:

```json
{
  "id": 1,
  "ticker": "AAA",
  "name": "CTCP Nhựa An Phát Xanh",
  "english_name": "An Phat Holdings",
  "exchange": "HSX",
  "tax_code": "0800373586",
  "website": "www.anphatbioplastics.com",
  "address": "Lô CN11 + CN12, Cụm Công nghiệp An Đồng, Huyện Nam Sách, Tỉnh Hải Dương",
  "phone": "+84 (320) 375-5998",
  "email": "anphat@anphatplastic.com",
  "founding_date": "2007-03-09",
  "sector_lv1": "Materials",
  "sector_lv2": "Chemicals",
  "sector_lv3": "Plastics",
  "sector_lv4": null,
  "margin_status": "Allowed",
  "margin_status_reason": null,
  "kl_ny": "382,274,496",
  "kl_lh": "382,274,496",
  "ngay_ny": "2016-11-25",
  "intro": "Leading manufacturer of biodegradable plastics in Vietnam.",
  "charter_cap": "3,822,744,960,000",
  "employee_num": "2861"
}
```

**Rust Struct:**
```rust
pub struct StockListItem {
    pub id: i32,
    pub ticker: String,
    pub name: Option<String>,
    pub english_name: Option<String>,
    pub exchange: Option<String>,
    pub tax_code: Option<String>,
    pub website: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub founding_date: Option<String>,
    pub sector_lv1: Option<String>,
    pub sector_lv2: Option<String>,
    pub sector_lv3: Option<String>,
    pub sector_lv4: Option<String>,
    pub margin_status: Option<String>,
    pub margin_status_reason: Option<String>,
    pub kl_ny: Option<String>,
    pub kl_lh: Option<String>,
    pub ngay_ny: Option<String>,
    pub intro: Option<String>,
    pub charter_cap: Option<String>,
    pub employee_num: Option<String>,
}
```

**Error Handling:**
If the ticker does not exist, the API will return a 404 or an empty object.

---

## Get Silver Price

**Endpoint:**
```
GET http://103.48.84.52:4040/silver-price
```

**Description:**
This endpoint returns the latest silver price information.

**Response Structure:**
The response is a JSON array of objects, each with the following fields:

```json
[
  {
    "date": 1710000000,
    "buy": 25000.5,
    "sell": 25500.0
  }
]
```

**Rust Struct:**
```rust
pub struct SilverPrice {
    pub date: i64,
    pub buy: f64,
    pub sell: f64,
}
```

**Field Notes:**
- `date`: Unix timestamp (seconds since epoch)
- `buy`: Buy price (floating-point number)
- `sell`: Sell price (floating-point number)

**Error Handling:**
If there is no silver price data, the API will return an empty array.






## Get Report info

**Endpoint**  
`GET http://103.48.84.52:4040/reports/{symbol}/{offset}/{number}`

**Description**  
  This endpoint returns a list of reports for the specified stock symbol, with pagination support.

**Parameters**  
  - `{symbol}`: The stock symbol to query reports for (e.g., AAA).  
  - `{offset}`: The starting index for pagination (e.g., 0).  
  - `{number}`: The number of reports to retrieve (e.g., 10).  

**Response Structure**  
  The response is a JSON array of objects, each with the following fields:

  ```json
  [
    {
      "report_id": 12345,
      "category_id": 1,
      "source_id": 2,
      "source_name": "Example Source",
      "sector_id": 3,
      "symbol": "AAA",
      "title": "Quarterly Report Q1 2025",
      "description": "Detailed financial report for Q1 2025.",
      "date": "2025-04-01T00:00:00",
      "pages": 25,
      "size": 1048576,
      "file_name": "Q1_2025_Report.pdf",
      "file_extension": "pdf",
      "language": "en",
      "downloads": 100,
      "is_hot": true
    },
    ...
  ]
  ```

**Data Structure**  
  ```rust
  pub struct Report {
      pub report_id: i64,
      pub category_id: Option<i32>,
      pub source_id: Option<i32>,
      pub source_name: Option<String>,
      pub sector_id: Option<i32>,
      pub symbol: Option<String>,
      pub title: Option<String>,
      pub description: Option<String>,
      pub date: Option<chrono::NaiveDateTime>,
      pub pages: Option<i32>,
      pub size: Option<i64>,
      pub file_name: Option<String>,
      pub file_extension: Option<String>,
      pub language: Option<String>,
      pub downloads: Option<i32>,
      pub is_hot: Option<bool>,
  }
  ```

  **Field Notes**  
  - `date`: ISO 8601 format (e.g., "2025-04-01T00:00:00").  
  - `size`: File size in bytes.  
  - `is_hot`: Indicates if the report is trending.  

  **Error Handling**  
  If the symbol does not exist or there are no reports, the API will return an empty array.






## Get Report PDF file

**Endpoint**  
`GET http://103.48.84.52:4040/report-file/{report_id}`  

**Parameters**  
- `{report_id}`: The unique ID of the report to download (e.g., 12345).

**Description**  
This endpoint returns a PDF file for the specified report ID.

**Response**  
The response is a binary PDF file. Ensure your application handles the file download appropriately.

**Error Handling**  
If the `report_id` does not exist, the API will return a 404 error.




## Get FINANCIAL RATIOS

**Endpoint**  
`GET http://103.48.84.52:4040/financial-data/{symbol}` 

**Description**  
This endpoint returns financial ratio data for the specified stock symbol.

**Parameters**  
- `{symbol}`: The stock symbol to query (e.g., AAA).

**Response Structure**  
The response is a JSON object with the following fields:

```json
{
  "symbol": "AAA",
  "year": 2025,
  "quarter": 1,
  "company_type": "Public",
  "icb_code": "1353",
  "icb_name": "Chemicals",
  "financial_values": {
    "ROE": 15.2,
    "ROA": 8.5,
    "DebtToEquity": 0.75,
    ...
  }
}
```

**Data Structure**  
```rust
pub struct FinancialData {
    pub symbol: String,
    pub year: i32,
    pub quarter: i32,
    pub company_type: String,
    pub icb_code: Option<String>,
    pub icb_name: Option<String>,
    pub financial_values: Value, // JSONB field
}
```

```json
"financialValues": {
            "Quarter": 1,
            "Year": 2025,
            "CompanyType": "General",
            "TotalRevenue": 16064980391264,
            "NetSale": 16058140942460,
            "CostOfGoodSold": 9756793038194,
            "GrossProfit": 6301347904266,
            "ProfitFromFinancialActivity": 261988123827,
            "ProfitFromAssociate": 136730336838,
            "FinancialExpense": 311343769364,
            "InterestExpense": 152389538333,
            "SellingExpense": 1828785929282,
            "ManagingExpense": 1877437056836,
            "NetProfitFromOperatingActivity": 2993843378813,
            "OtherRevenue": 39212521539,
            "OtherExpense": 8362389503,
            "OtherProfit": 30850132036,
            "ProfitBeforeTax": 3024693510849,
            "CorporateIncomeTax": 377287599563,
            "ProfitAfterTax": 2595557480309,
            "ParentCompanyShareholderProfitAfterTax": 2174301386525,
            "CoreEBIT": 3009502580308,
            "EBIT": 3177083049182,
            "EBITDA": 3821106170828,
            "CoreEBITDA": 3653525701954,
            "EBIT_TTM": 12128145367172,
            "EBITDA_TTM": 14714855049957,
            "NetSale_TTM": 64814006880129,
            "ParentCompanyShareholderProfitAfterTax_TTM": 8225305588585,
            "TotalRevenue_TTM": 64934335128424,
            "GrossProfit_TTM": 24590634243496,
            "FinancialActivityRevenue_TTM": 2045711817013,
            "FinancialExpense_TTM": 1788825878044,
            "InterestExpense_TTM": 566195180033,
            "SellingExpense_TTM": 6665214818354,
            "ManagingExpense_TTM": 7133463057836,
            "NetProfitFromOperatingActivity_TTM": 11504569341759,
            "OtherExpense_TTM": 84954146257,
            "OtherProfit_TTM": 57380845380,
            "ProfitBeforeIncomeTax_TTM": 11561950187139,
            "ProfitBeforeTax_TTM": 11561950187139,
            "CorporateIncomeTax_TTM": 1706579474608,
            "ProfitAfterIncomeTax_TTM": 9855370712531,
            "ProfitAfterTax_TTM": 9855370712531,
            "ProfitFromFinancialActivity_TTM": 256885938969,
            "CostOfGoodSold_TTM": 40223372636633,
            "DepreciationAndAmortisation_TTM": 2586709682785,
            "OPEX_TTM": 57602410011732,
            "TotalRevenue_CUM": 16064980391264,
            "NetSale_CUM": 16058140942460,
            "GrossProfit_CUM": 6301347904266,
            "FinancialActivitiesRevenue_CUM": 573331893191,
            "FinancialExpense_CUM": 311343769364,
            "InterestExpense_CUM": 152389538333,
            "SellingExpense_CUM": 1828785929282,
            "ManagingExpense_CUM": 1877437056836,
            "NetProfitFromOperatingActivity_CUM": 2993843378813,
            "OtherExpense_CUM": 8362389503,
            "OtherProfit_CUM": 30850132036,
            "ProfitBeforeIncomeTax_CUM": 3024693510849,
            "ProfitBeforeTax_CUM": 3024693510849,
            "CorporateIncomeTax_CUM": 377287599563,
            "ProfitAfterIncomeTax_CUM": 2595557480309,
            "ProfitAfterTax_CUM": 2595557480309,
            "ParentCompanyShareholderProfitAfterTax_CUM": 2174301386525,
            "OPEX_CUM": 14211858213719,
            "ProfitFromFinancialActivity_CUM": 261988123827,
            "CostOfGoodSold_CUM": 9756793038194,
            "TotalCurrentAsset": 46075510989597,
            "Cash": 5342746710936,
            "CashEquivalent": 1412898503316,
            "ShortTermFinancialInvestment": 23768377985785,
            "TotalShortTermReceivable": 11997831491115,
            "ShortTermAccountReceivable": 10855474371546,
            "TotalInventory": 2107939084995,
            "ShortTermPrepaidExpense": 664880944825,
            "TotalOtherShortTermAsset": 1445717213450,
            "TotalNonCurrentAsset": 27922162132192,
            "TotalLongTermReceivable": 433719282180,
            "LongTermAccountReceivable": 0,
            "LongTermFinancialInvestment": 3526503466795,
            "FixedAsset": 15548353983782,
            "TangibleAsset": 13567762652609,
            "IntangibleAsset": 1978640933934,
            "RealEstateInvest": 0,
            "InProgressLongTermAsset": 2812693461694,
            "TotalOtherNonCurrentAsset": 4534789819867,
            "GoodWill": 1066102117874,
            "TotalDebt": 36101018424847,
            "TotalShortTermDebt": 33917117422503,
            "ShortTermInterestBearingDebt": 18320209599285,
            "ShortTermAccountPayable": 3244393302947,
            "TotalLongTermDebt": 2183901002344,
            "LongTermInterestBearingDebt": 987681080191,
            "LongTermAccountPayable": 0,
            "ConvertibleBond": 0,
            "StockHolderEquity": 37893904696942,
            "TotalStockHolderEquity": 37896654696942,
            "PaidInCapital": 14710691830000,
            "ContributedCapitalOfShareHolder": 14710691830000,
            "AccumulatedDeficit": 13205526456323,
            "TreasuryStock": 0,
            "RetainedProfit": 2174301386525,
            "TotalCapital": 73997673121789,
            "TotalAsset": 73997673121789,
            "ChangeInWorkingCapital": null,
            "WorkingCapital": 12158393567094,
            "AvgTotalAsset": 68011189085109,
            "AvgTotalCurrentAsset": 41877330012043,
            "AvgCash": 5377333538446,
            "AvgCashEquivalent": 1171217821178,
            "AvgTotalShortTermReceivable": 11026351428666,
            "AvgShortTermAccountReceivable": 10124619215251,
            "AvgTotalInventory": 1982171363232,
            "AvgTotalLongTermReceivable": 376695984364,
            "AvgLongTermAccountReceivable": 0,
            "AvgFixedAsset": 14681295617731,
            "AvgTangibleAsset": 12932769651053,
            "AvgIntangibleAsset": 1745779844293,
            "AvgRealEstateInvest": 0,
            "AvgInProgressLongTermAsset": 2121028012509,
            "AvgLongTermFinancialInvestment": 3471031311014,
            "AvgTotalOtherNonCurrentAsset": 4144895369994,
            "AvgTotalDebt": 33199311221056,
            "AvgTotalShortTermDebt": 31475150184140,
            "AvgShortTermInterestBearingDebt": 17152148146811,
            "AvgShortTermAccountPayable": 3212031912906,
            "AvgTotalLongTermDebt": 1724161036916,
            "AvgLongTermAccountPayable": 0,
            "AvgLongTermInterestBearingDebt": 756415033658,
            "AvgConvertibleBond": 0,
            "AvgStockHolderEquity": 34809127864053,
            "AvgTotalStockHolderEquity": 34811877864053,
            "AvgTreasuryStock": 0,
            "AvgAccumulatedDeficit": 11842182611885,
            "AvgRetainedProfit": 1986183903473,
            "AvgTotalCapital": 68011189085109,
            "AvgTotalAccountReceivable": 10124619215251,
            "AvgWorkingCapital": 10402179827903,
            "AvgCapitalEmployed": 36536038900969,
            "AvgInvestedCapital": 46171889684898,
            "Amortization": null,
            "Depreciation": null,
            "CAPEX": 6213790541298,
            "DepreciationAndAmortization": 644023121646,
            "CashflowFromOperatingActivity": -2506903722401,
            "CashflowFromInvestingActivity": -4397389001760,
            "CashflowFromFinancingActivity": 4273045645176,
            "FCF": null,
            "EffectOfForeignExchangeDifference": 71451854353,
            "CashAndCashEquivalentAtTheBeginningOfPeriod": 9315440438884,
            "CashAndCashEquivalentAtTheEndOfPeriod": 6755645214252,
            "AvgShareInPeriod": 1471069183,
            "ShareAtPeriodEnd": 1471069183,
            "AvgPriceInPeriod": 134865.86345,
            "PriceAtPeriodEnd": 121000,
            "AvgMarketCapInPeriod": 198397015565600,
            "MarketCapAtPeriodEnd": 177999371143000,
            "PE": 21.64046,
            "PS": 2.74631,
            "PB": 5.54964,
            "TangibleTPB": 5.9145,
            "SalePerShare": 44059.11539,
            "BookValuePerShare": 21803.22729,
            "TangibleBookValuePerShare": 20458.19135,
            "BasicEPS": 5591.37917,
            "DilutedEPS": 5591.37917,
            "EVOverEBITDA": 13.3453,
            "GrossMargin": 0.3794,
            "EBITMargin": 0.18712,
            "OperatingMargin": 0.1775,
            "PreTaxMargin": 0.17839,
            "ROS": 0.12691,
            "FinancialProfitOverProfitBeforeTax": 0.02222,
            "ROE": 0.23628,
            "ROA": 0.12094,
            "ROCE": 0.33195,
            "ROIC": 0.21914,
            "QuickRatio": 1.2537,
            "CurrentRatio": 1.35847,
            "CashRatio": 0.19918,
            "InterestCoverageRatio": 21.4204,
            "LongtermDebtOverEquity": 0.05763,
            "TotalDebtOverEquity": 0.95262,
            "TotalDebtOverAsset": 0.48787,
            "TotalAssetTurnover": 0.95299,
            "InventoryTurnover": 18.99358,
            "ReceivableTurnover": 6.40162,
            "CurrentAssetTurnover": 1.54771,
            "FixedAssetTurnover": 4.41473,
            "SectorPE": 20.6212,
            "SectorPB": 5.15761,
            "SectorTangiblePB": 5.46702,
            "SectorPS": 2.64988,
            "SectorEPS": 5544.01329,
            "SectorEPS_AVG": 2432.95709,
            "SectorROS": 0.1285,
            "SectorGrossMargin": 0.37689,
            "SectorEBITMargin": 0.19108,
            "SectorOperatingMargin": 0.17905,
            "SectorQuickRatio": 1.21071,
            "SectorCurrentRatio": 1.40644,
            "SectorInterestCoverageRatio": 17.35603,
            "SectorTotalDebtOverEquity": 1.02029,
            "SectorROE": 0.23193,
            "SectorROA": 0.11429,
            "SectorROCE": 0.30842,
            "SectorROIC": 0.20398,
            "SectorTotalAssetTurnover": 0.88936,
            "SectorInventoryTurnover": 7.95774,
            "SectorReceivableTurnover": 6.36778,
            "SectorCurrentAssetTurnover": 1.42498,
            "SectorFixedAssetTurnover": 4.61176,
            "SectorEVOverEBITDA": 13.77868,
            "SectorNetSales": 17132664335269,
            "SectorNetSale": 17132664335269,
            "SectorGrossProfits": 6843188876723,
            "SectorGrossProfit": 6843188876723,
            "SectorNetProfitsFromOperatingActivities": 3439607030958,
            "SectorNetProfitFromOperatingActivity": 3439607030958,
            "SectorProfitsBeforeIncomeTaxes": 3472634402339,
            "SectorProfitBeforeTax": 3472634402339,
            "SectorProfitsAfterIncomeTaxes": 2943258115710,
            "SectorProfitAfterIncomeTax": 2943258115710,
            "SectorProfitAfterTax": 2943258115710,
            "SectorTotalCurrentAsset": 52397640779806,
            "SectorTotalInventory": 5778366791650,
            "SectorInventory": 5909308311312,
            "SectorTotalOtherShortTermAsset": 1513820111089,
            "SectorTotalShortTermDebt": 37255418236748,
            "SectorTotalLongTermDebt": 4846845984341,
            "SectorTotalStockHolderEquity": 41265068025920,
            "SectorTotalDebt": 42102264221089,
            "SectorTotalAsset": 83367332247009,
            "SectorIntangibleAsset": 1990384554549,
            "SectorCash": 5463847746701,
            "SectorCashEquivalent": 1470967539551,
            "SectorFixedAsset": 15711191777725,
            "SectorCostOfGoodSold": 10289475458546,
            "SectorInterestExpense": 176042417291,
            "SectorTangibleAsset": 13679850910187,
            "SectorTotalShortTermReceivable": 14073709694523,
            "SectorShortTermAccountReceivable": 11519855436510,
            "SectorInProgressLongTermAsset": 3525279877495,
            "SectorInProgressConstructionCost": 3525258222329,
            "SectorShortTermInterestBearingDebt": 20178814320137,
            "SectorLongTermInterestBearingDebt": 3267577874369,
            "SectorStockHolderEquity": 41262318025920,
            "SectorPaidInCapital": 16840851510000,
            "ICBCode": "10101015",
            "ICBName": "Công nghệ phần mềm                                                                                                                                                                                                                                              ",
            "DilutedEPSGrowth": 0.0481,
            "DilutedEPSGrowth_TTM": 0.0481,
            "DilutedEPSGrowth_LFY": null,
            "DilutedEPSGrowth_03Yr": null,
            "BasicEPSGrowth": 0.0481,
            "BasicEPSGrowth_TTM": 0.0481,
            "BasicEPSGrowth_LFY": null,
            "BasicEPSGrowth_03Yr": null,
            "ProfitGrowth": 0.20927,
            "ParentCompanyShareholderProfitAfterTaxGrowth": 0.20927,
            "ProfitGrowth_MRQ": 0.20927,
            "ProfitGrowth_MRQ2": 0.20747,
            "ProfitGrowth_TTM": 0.21407,
            "ProfitGrowth_LFY": null,
            "ProfitGrowth_03Yr": null,
            "SaleGrowth": 0.13945,
            "SalesGrowth_MRQ": 0.13945,
            "SalesGrowth_MRQ2": 0.19859,
            "SalesGrowth_TTM": 0.17781,
            "SaleGrowth_LFY": null,
            "SaleGrowth_03Yr": null,
            "ProfitBeforeTaxGrowth": 0.20237,
            "ProfitBeforeTaxGrowth_MRQ": 0.20237,
            "ProfitBeforeTaxGrowth_MRQ2": 0.20296,
            "ProfitBeforeTaxGrowth_TTM": 0.20355,
            "ProfitBeforeTaxGrowth_LFY": null,
            "ProfitBeforeTaxGrowth_03Yr": null,
            "ProfitAfterTaxGrowth": 0.21029,
            "ProfitAfterTaxGrowth_MRQ": 0.21029,
            "ProfitAfterTaxGrowth_MRQ2": 0.20887,
            "ProfitAfterTaxGrowth_TTM": 0.21394,
            "ProfitAfterTaxGrowth_LFY": null,
            "ProfitAfterTaxGrowth_03Yr": null,
            "TotalAssetGrowth": 0.19304,
            "TotalAssetGrowth_YoY": 0.19304,
            "TotalAssetGrowth_QoQ": null,
            "TotalAssetGrowth_LFY": null,
            "TotalAssetGrowth_03Yr": null,
            "CurrentAssetGrowth": 0.22284,
            "CurrentAssetGrowth_YoY": 0.22284,
            "CurrentAssetGrowth_QoQ": null,
            "CurrentAssetGrowth_LFY": null,
            "CurrentAssetGrowth_03Yr": null,
            "FixedAssetGrowth": 0.12553,
            "FixedAssetGrowth_YoY": 0.12553,
            "FixedAssetGrowth_QoQ": null,
            "FixedAssetGrowth_LFY": null,
            "FixedAssetGrowth_03Yr": null,
            "EquityGrowth": 0.19447,
            "EquityGrowth_YoY": 0.19447,
            "EquityGrowth_QoQ": null,
            "EquityGrowth_LFY": null,
            "EquityGrowth_03Yr": null,
            "CashDividend": 1000.0,
            "StockDividend": 0.15,
            "RetentionRatio": 1.0,
            "DividendYield": 0.00892,
            "TotalStockReturn": -0.25592,
            "CapitalGainsYield": -0.2625,
            "PayoutRatio": 0.0,
            "PlanningRevenue": 75400000000000,
            "PlanningNetSale": 75400000000000,
            "PlanningProfitBeforeTax": 13395000000000,
            "PlanningProfitAfterTax": 10716000000000,
            "PlanningCashDividend": 2000.0,
            "PlanningStockDividend": 0.0,
            "PlanningEPS": 9100.0,
            "ROAScore": null,
            "CFOScore": null,
            "DeltaROAScore": null,
            "AccrualScore": null,
            "DeltaLeverScore": null,
            "DeltaLiquidScore": null,
            "EQOfferScore": null,
            "DeltaMarginScore": null,
            "DeltaTurnScore": null,
            "PiotroskiFScore": null,
            "ManufacturingZScore": null,
            "NonManufacturingZScore": null,
            "ManufacturingStatus": null,
            "NonManufacturingStatus": null,
            "ManufacturingSPRating": null,
            "NonManufacturingSPRating": null,
            "ManufacturingMoodyRating": null,
            "NonManufacturingMoodyRating": null,
            "WorkingCapitalOverAsset": 0.16431,
            "RetainedEarningOverAsset": 0.17846,
            "EBITOverAsset": 0.1639,
            "MarketCapOverDebt": 4.93059,
            "NetSaleOverAsset": 0.87589
        }
```
**Field Notes**  
- `financial_values`: A JSON object containing key-value pairs for various financial ratios (e.g., ROE, ROA, DebtToEquity).  
- `year` and `quarter`: Indicate the period for the financial data.  

**Error Handling**  
If the symbol does not exist or has no financial data, the API will return an empty object.

## COMPANY COMPARE BY SECTOR

**Endpoint:**
```
GET /company-compare/{symbol}
```
**Description**
Returns a list of companies in the same ICB sector as the requested symbol, including key financial metrics for comparison.
The last item in the list is the sector average.

**Body**
```json
[
  {
    "symbol": "HPG",
    "companyName": "CTCP Hòa Phát",
    "market_cap": 9315440438884,
    "PE": 21.64046,
    "PS": 2.74631,
    "PB": 5.54964,
    "BasicEPS": 5591.37917,
    "ROE": 0.23628,
    "ROA": 0.12094,
    "EVOverEBITDA": 13.3453,
    "NetSale": 16058140942460,
    "ProfitAfterTax": 2595557480309
  },
  {
    "symbol": "HSG",
    "companyName": "Tập đoàn Hoa Sen",
    "market_cap": 3315440438884,
    "PE": 12.64046,
    "PS": 3.74631,
    "PB": 2.4,
    "BasicEPS": 1591.37917,
    "ROE": 0.228,
    "ROA": 0.1094,
    "EVOverEBITDA": 13.3453,
    "NetSale": 16058140942460,
    "ProfitAfterTax": 2595557480309
  },
  // ... more companies ...
  {
    "symbol": "AVERAGE",
    "companyName": "Trung bình ngành",
    "market_cap": null,
    "SectorPE": 20.6212,
    "SectorPS": 2.64988,
    "SectorPB": 5.15761,
    "SectorEPS": 5544.01329,
    "SectorROE": 0.23193,
    "SectorROA": 0.11429,
    "SectorEVOverEBITDA": 13.77868,
    "SectorNetSales": 17132664335269,
    "SectorProfitAfterTax": 2943258115710
  }
]
```
---
## TOP STOCK INFLUENCE

**Endpoint:**
```
GET http://103.48.84.52:4040/top-stock-influence
```
**Description:**
This endpoint provide ranking for stock that have most influence on index, use row_num to rank.
1 for most positive 2 3 4 ... 20 most negative

**Rust Struct:**
```rust
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct TopStockInfluence {
    pub stock_code: String,
    pub close_price: Option<i64>,
    pub change: Option<f64>,
    pub per_change: Option<f64>,
    pub klcplh: Option<i64>,
    pub market_cap: Option<i64>,
    pub weight: Option<f64>,
    pub basic_index: Option<f64>,
    pub influence_percent: Option<f64>,
    pub influence_index: Option<f64>,
    pub order_type: Option<i32>,
    pub row_num: Option<i32>,
}
```
## GET EXCHANGE INDEX
**Endpoint:**
```
GET http://103.48.84.52:4040/exchange-index
```
**Description**
Returns exchange index data for all available exchanges.
Authentication is required (provide a valid token in the Authorization header).
**Body**
```json
[
        {
            "latestValue": [],
            "exchange": "hose",
            "indexId": "VNINDEX",
            "indexValue": 1624.61,
            "prevIndexValue": 1624.53,
            "time": 1757401475083,
            "advances": 93,
            "allQty": 852912436,
            "allValue": 23841120048960,
            "ceiling": 5,
            "chartHigh": 1632.89,
            "chartLow": 1613.01,
            "declines": 217,
            "firstM1Seq": 7,
            "floor": 3,
            "lastM1Seq": 25592,
            "nochanges": 57,
            "timeMaker": 1757401445080,
            "totalQtty": 787226684,
            "totalQttyPT": 65685752,
            "totalValue": 21978850587510,
            "totalValuePT": 1862269461450,
            "change": 0.08,
            "changePercent": 0,
            "chartOpen": 1629.58,
            "label": "VNINDEX",
            "exchangeLabel": "HOSE",
            "totalBuyForeignQtty": 125715719,
            "totalSellForeignQtty": 193421356
        },
        {
            "latestValue": [],
            "exchange": "hose",
            "indexId": "VN30",
            "indexValue": 1811.8,
            "prevIndexValue": 1807.22,
            "time": 1757401475072,
            "advances": 11,
            "allQty": 347920654,
            "allValue": 11856587677550,
            "ceiling": 0,
            "change": 4.58,
            "changePercent": 0.25,
            "chartHigh": 1821.44,
            "chartLow": 1795.82,
            "declines": 17,
            "firstM1Seq": 2,
            "floor": 0,
            "lastM1Seq": 25587,
            "nochanges": 2,
            "timeMaker": 1757401440072,
            "totalQtty": 326003246,
            "totalQttyPT": 21917408,
            "totalValue": 11291192692650,
            "totalValuePT": 565394984900,
            "chartOpen": 1814.42,
            "label": "VN30",
            "exchangeLabel": "VN30",
            "totalBuyForeignQtty": 125715719,
            "totalSellForeignQtty": 193421356
        },
...
```

**Rust Struct:**
```rust
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SsiExchangeIndexItem {
    pub exchange: Option<String>,
    pub indexId: Option<String>,
    pub indexValue: Option<f64>,
    pub prevIndexValue: Option<f64>,
    pub time: Option<i64>,
    pub advances: Option<u32>,
    pub allQty: Option<u64>,
    pub allValue: Option<u64>,
    pub ceiling: Option<u32>,
    pub chartHigh: Option<f64>,
    pub chartLow: Option<f64>,
    pub declines: Option<u32>,
    pub firstM1Seq: Option<u32>,
    pub floor: Option<u32>,
    pub lastM1Seq: Option<u32>,
    pub nochanges: Option<u32>,
    pub timeMaker: Option<i64>,
    pub totalQtty: Option<u64>,
    pub totalQttyPT: Option<u64>,
    pub totalValue: Option<u64>,
    pub totalValuePT: Option<u64>,
    pub change: Option<f64>,
    pub changePercent: Option<f64>,
    pub chartOpen: Option<f64>,
    pub label: Option<String>,
    pub exchangeLabel: Option<String>,
    pub totalBuyForeignQtty: Option<u64>,
    pub totalSellForeignQtty: Option<u64>,
}
```

---



## LOGIN ROUTE

**Endpoint:**
```
POST http://103.48.84.52:4040/login
```

**Body**
```json
{
  "username": "your_username",
  "password": "your_password"
}
```
**Description:**
Authenticate a user and return a session token for API access.
Implements rate limiting to prevent brute-force attacks.

**Response Structure:**

Success (HTTP 200):
```json
{
  "token": "JWT_TOKEN_STRING",
  "user_id": 123,
  "username": "user123",
  "email": "user123@example.com",
  "phone": "0123456789",
  "session_id": "SESSION_ID_STRING"
}
```
Rate Limit Exceeded (HTTP 429):
```json
{
  "error": "Too many login attempts. Please try again later."
}
```

**Rust Struct:**
```rust
#[derive(Deserialize, Debug)]
struct TokenResponse {
    token: String,
    user_id: i32,
    username: String,
    email: Option<String>,
    phone: Option<String>,
    session_id: String,
}
```

**Notes for Developers:**
On success, use the returned token in the Authorization header for subsequent API requests:
Rate limiting is enforced per IP address.
All errors are returned as JSON.

---


## REGISTER ROUTE

**Endpoint:**
```
POST http://103.48.84.52:4040/register
```

**Body**
```json
{
  "username": "your_username",
  "email": "your_email@example.com",
  "phone": "0123456789",
  "password": "your_password"
}
```
**Description:**
Creates a new user account and returns a session token for API access.
Implements rate limiting to prevent abuse.

**Response Structure:**

Success (HTTP 200):
```json
{
  "token": "JWT_TOKEN_STRING",
  "user_id": 123,
  "username": "your_username",
  "email": "your_email@example.com",
  "phone": "0123456789",
  "session_id": "SESSION_ID_STRING"
}
```
Rate Limit Exceeded (HTTP 429):
```json
{
  "error": "Too many login attempts. Please try again later."
}
```

**Rust Struct:**
```rust
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub email: Option<String>,
    pub phone: Option<String>,
}
```

**Notes for Developers:**
On success, use the returned token in the Authorization header for subsequent API requests:
Rate limiting is enforced per IP address.
All errors are returned as JSON.

---
























