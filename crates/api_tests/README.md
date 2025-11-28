# API Tests - Scalable API Testing Framework

A comprehensive, scalable API testing framework built in Rust for testing REST APIs with authentication, request builders, test utilities, and domain-specific API modules.

## 🏗️ Architecture

```
api_tests/
├── src/
│   ├── client.rs           # HTTP client with auth support
│   ├── auth.rs             # Authentication manager
│   ├── models/             # Data models
│   │   ├── lead.rs         # Lead domain models
│   │   ├── invoice.rs      # Invoice domain models
│   │   └── common.rs       # Common response models
│   ├── api/                # API endpoint modules
│   │   ├── auth_api.rs     # Authentication endpoints
│   │   ├── leads_api.rs    # Leads CRUD endpoints
│   │   └── invoices_api.rs # Invoices CRUD endpoints
│   ├── utils/              # Test utilities
│   │   ├── assertions.rs   # Response assertions
│   │   ├── factories.rs    # Test data factories
│   │   └── retry.rs        # Retry with backoff
│   └── tests/              # Test suites
│       ├── auth_tests.rs   # Auth flow tests
│       ├── leads_tests.rs  # Leads API tests
│       └── invoices_tests.rs # Invoices API tests
└── Cargo.toml
```

## ✨ Features

- **🔐 Authentication**: Automatic token management and injection
- **🔄 Retry Logic**: Exponential backoff for flaky requests
- **🏭 Data Factories**: Generate realistic test data with faker
- **✅ Assertions**: Rich assertion helpers for API responses
- **📦 Type Safety**: Strongly-typed request/response models
- **⚡ Async/Await**: Full async support for parallel execution
- **🎯 Domain-Specific APIs**: Clean, modular API client design
- **📊 Pagination**: Built-in support for paginated responses

## 🚀 Getting Started

### Prerequisites

- Rust 1.70 or higher
- Environment variables configured in `.env`:
  ```env
  WMS_EMAIL=your_email@example.com
  WMS_PASSWORD=your_password
  WMS_PHONE=your_phone
  WMS_OTP=your_otp
  ELDER_NUMBER=elder_number
  ```

### Running Tests

```bash
# Run all tests
cargo test -p api_tests

# Run specific test module
cargo test -p api_tests --test auth_tests
cargo test -p api_tests --test leads_tests
cargo test -p api_tests --test invoices_tests

# Run with output
cargo test -p api_tests -- --nocapture

# Run specific test
cargo test -p api_tests test_create_lead -- --nocapture
```

## 📚 Usage Examples

### Basic Authentication

```rust
use api_tests::{AuthManager, AuthApi};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Login with config credentials
    let response = AuthManager::login_with_config().await?;
    println!("Token: {}", response.token);
    
    // Or login with specific credentials
    let auth = AuthApi::new();
    let response = auth.login(
        "user@example.com".to_string(),
        "password".to_string()
    ).await?;
    
    Ok(())
}
```

### Creating a Lead

```rust
use api_tests::{AuthManager, LeadsApi, LeadFactory};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Authenticate first
    AuthManager::login_with_config().await?;
    
    // Generate test data
    let lead_request = LeadFactory::create_request();
    
    // Create lead
    let api = LeadsApi::new();
    let lead = api.create(lead_request).await?;
    
    println!("Created lead: {} (ID: {})", lead.name, lead.id);
    
    Ok(())
}
```

### Creating an Invoice

```rust
use api_tests::{AuthManager, InvoicesApi, InvoiceFactory, InvoiceItem};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    AuthManager::login_with_config().await?;
    
    // Create invoice with custom items
    let items = vec![
        InvoiceItem::new("Consulting".to_string(), 5, 150.0),
        InvoiceItem::new("Support".to_string(), 1, 500.0),
    ];
    
    let invoice_request = InvoiceFactory::with_items(items);
    
    let api = InvoicesApi::new();
    let invoice = api.create(invoice_request).await?;
    
    println!("Invoice: {} - Total: ${:.2}", 
        invoice.invoice_number, invoice.total);
    
    Ok(())
}
```

### Search and Filter

```rust
use api_tests::{LeadsApi, LeadStatus, LeadSearchParams};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api = LeadsApi::new();
    
    // Search by name
    let results = api.search_by_name("John Doe").await?;
    println!("Found {} leads", results.items.len());
    
    // Filter by status
    let new_leads = api.get_by_status(LeadStatus::New).await?;
    println!("Found {} new leads", new_leads.items.len());
    
    // Custom search params
    let params = LeadSearchParams {
        city: Some("Mumbai".to_string()),
        status: Some(LeadStatus::Qualified),
        page: Some(1),
        limit: Some(20),
        ..Default::default()
    };
    let filtered = api.list(Some(params)).await?;
    
    Ok(())
}
```

### Using Assertions

```rust
use api_tests::{LeadsApi, utils::*};
use reqwest::StatusCode;

#[tokio::test]
async fn test_with_assertions() -> anyhow::Result<()> {
    let api = LeadsApi::new();
    let response = api.client.get("/api/leads").send().await?;
    
    // Assert status code
    assert_ok(response.status())?;
    
    // Assert response time
    assert_response_time(elapsed_ms, 1000)?;
    
    Ok(())
}
```

### Using Retry Logic

```rust
use api_tests::utils::{retry_async, retry_times, RetryConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Retry with default config (3 attempts, exponential backoff)
    let result = retry_times(3, || async {
        // Your async operation here
        Ok::<_, anyhow::Error>(())
    }).await?;
    
    // Custom retry configuration
    let config = RetryConfig::default()
        .with_attempts(5)
        .with_initial_delay(200)
        .with_max_delay(10000);
    
    retry_async(config, || async {
        // Your operation
        Ok::<_, anyhow::Error>(())
    }).await?;
    
    Ok(())
}
```

## 🧪 Test Data Factories

The framework includes powerful test data factories that generate realistic fake data:

```rust
use api_tests::{LeadFactory, InvoiceFactory, TestDataFactory};

// Generate complete lead request
let lead = LeadFactory::create_request();

// Generate lead with specific fields
let lead = LeadFactory::with_name("John Doe");
let lead = LeadFactory::with_phone("1234567890");

// Generate minimal lead (required fields only)
let lead = LeadFactory::minimal();

// Generate invoice
let invoice = InvoiceFactory::create_request();
let invoice = InvoiceFactory::with_customer("Jane Smith");

// Generate random data
let email = TestDataFactory::email();
let phone = TestDataFactory::phone();
let random_str = TestDataFactory::random_string(10);
let num = TestDataFactory::random_number(1, 100);
```

## 🎯 Best Practices

1. **Always authenticate before API calls**
   ```rust
   AuthManager::login_with_config().await?;
   ```

2. **Use factories for test data**
   ```rust
   let lead = LeadFactory::create_request();
   ```

3. **Handle errors gracefully in tests**
   ```rust
   match api.create(request).await {
       Ok(result) => { /* assertions */ }
       Err(e) => println!("⚠ Endpoint not available: {}", e),
   }
   ```

4. **Clean up test data**
   ```rust
   // Create
   let lead = api.create(request).await?;
   
   // Test operations
   // ...
   
   // Clean up
   api.delete(&lead.id).await?;
   ```

5. **Use descriptive test names**
   ```rust
   #[tokio::test]
   async fn test_create_lead_with_minimal_fields() -> Result<()> {
       // ...
   }
   ```

## 🔧 Configuration

The framework uses environment-based configuration via YAML files:

**config/dev.yaml**:
```yaml
env: "dev"
base_url: "https://api.example.com"
api_base_url: "https://api.example.com/api"
# Other config...
```

Environment variables override YAML configuration for sensitive data.

## 📊 Example Test Suite

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_full_lead_lifecycle() -> Result<()> {
        // Setup
        setup_auth().await?;
        let api = LeadsApi::new();
        
        // Create
        let request = LeadFactory::create_request();
        let created = api.create(request).await?;
        assert!(!created.id.is_empty());
        
        // Read
        let retrieved = api.get(&created.id).await?;
        assert_eq!(retrieved.id, created.id);
        
        // Update
        let update = UpdateLeadRequest {
            status: Some(LeadStatus::Contacted),
            ..Default::default()
        };
        let updated = api.update(&created.id, update).await?;
        assert_eq!(updated.status, LeadStatus::Contacted);
        
        // Delete
        api.delete(&created.id).await?;
        
        Ok(())
    }
}
```

## 🛠️ Dependencies

- `reqwest` - HTTP client
- `serde` / `serde_json` - Serialization
- `tokio` - Async runtime
- `anyhow` - Error handling
- `chrono` - Date/time handling
- `fake` - Fake data generation
- `rand` - Random data

## 📝 Contributing

When adding new API endpoints:

1. Add data models in `models/`
2. Create API module in `api/`
3. Add test suite in `tests/`
4. Update factories in `utils/factories.rs`
5. Add usage examples to this README

## 🐛 Troubleshooting

### Tests failing with authentication errors
- Ensure `.env` file has correct credentials
- Verify API endpoint URLs in `config/dev.yaml`

### Connection timeouts
- Check network connectivity
- Verify API server is running
- Adjust timeout in `client.rs` if needed

### Test data conflicts
- Use unique identifiers (timestamps, UUIDs)
- Clean up test data after each test
- Use isolated test environments

## 📄 License

Part of the Emoha QA automation project.
