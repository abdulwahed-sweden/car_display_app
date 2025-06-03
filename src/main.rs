use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
struct Metadata {
    title: String,
    description: String,
    version: String,
    author: String,
    repository: String,
    generated_date: String,
    data_source: String,
    standards: Vec<String>,
    total_codes: u32,
    code_breakdown: CodeBreakdown,
    categories: Vec<String>,
    systems: Vec<String>,
    system_breakdown: serde_json::Value,
    severity_levels: Vec<String>,
    severity_breakdown: SeverityBreakdown,
    code_type_descriptions: serde_json::Value,
}

#[derive(Debug, Deserialize, Serialize)]
struct CodeBreakdown {
    #[serde(rename = "P_codes")]
    p_codes: u32,
    #[serde(rename = "B_codes")]
    b_codes: u32,
    #[serde(rename = "C_codes")]
    c_codes: u32,
    #[serde(rename = "U_codes")]
    u_codes: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct SeverityBreakdown {
    #[serde(rename = "Info")]
    info: u32,
    #[serde(rename = "Critical")]
    critical: u32,
    #[serde(rename = "Warning")]
    warning: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct DiagnosticCode {
    code: String,
    description: String,
    category: String,
    system: String,
    severity: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct OdbDatabase {
    metadata: Metadata,
    diagnostic_codes: Vec<DiagnosticCode>,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body(r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>OBD Diagnostic Trouble Codes Database</title>
            <style>
                body {
                    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
                    margin: 0;
                    padding: 0;
                    background: #f5f5f5;
                }
                .container {
                    max-width: 1200px;
                    margin: 0 auto;
                    padding: 20px;
                }
                .hero {
                    background: linear-gradient(135deg, #1e3c72 0%, #2a5298 100%);
                    color: white;
                    padding: 60px 20px;
                    text-align: center;
                    border-radius: 10px;
                    margin-bottom: 30px;
                    box-shadow: 0 4px 6px rgba(0,0,0,0.1);
                }
                h1 {
                    margin: 0 0 20px 0;
                    font-size: 2.5em;
                }
                .description {
                    font-size: 1.2em;
                    opacity: 0.9;
                    margin-bottom: 30px;
                }
                .features {
                    display: grid;
                    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
                    gap: 20px;
                    margin: 40px 0;
                }
                .feature-card {
                    background: white;
                    padding: 30px;
                    border-radius: 10px;
                    text-align: center;
                    box-shadow: 0 2px 5px rgba(0,0,0,0.1);
                    transition: transform 0.3s;
                }
                .feature-card:hover {
                    transform: translateY(-5px);
                    box-shadow: 0 5px 15px rgba(0,0,0,0.2);
                }
                .feature-icon {
                    font-size: 3em;
                    margin-bottom: 15px;
                }
                .cta-button {
                    display: inline-block;
                    background: #ff6b6b;
                    color: white;
                    padding: 15px 40px;
                    text-decoration: none;
                    border-radius: 5px;
                    font-size: 1.2em;
                    transition: background 0.3s;
                }
                .cta-button:hover {
                    background: #ff5252;
                }
                .info-grid {
                    display: grid;
                    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
                    gap: 15px;
                    margin: 30px 0;
                }
                .info-box {
                    background: rgba(255,255,255,0.2);
                    padding: 15px;
                    border-radius: 5px;
                    text-align: center;
                }
                .info-number {
                    font-size: 2em;
                    font-weight: bold;
                }
                .info-label {
                    opacity: 0.9;
                }
            </style>
        </head>
        <body>
            <div class="container">
                <div class="hero">
                    <h1>OBD Diagnostic Trouble Codes Database</h1>
                    <p class="description">Comprehensive database of automotive diagnostic trouble codes (DTCs) based on SAE J2012 and ISO 15031 standards</p>
                    
                    <div class="info-grid">
                        <div class="info-box">
                            <div class="info-number">4,316</div>
                            <div class="info-label">Total Codes</div>
                        </div>
                        <div class="info-box">
                            <div class="info-number">4</div>
                            <div class="info-label">Categories</div>
                        </div>
                        <div class="info-box">
                            <div class="info-number">22</div>
                            <div class="info-label">Systems</div>
                        </div>
                        <div class="info-box">
                            <div class="info-number">3</div>
                            <div class="info-label">Severity Levels</div>
                        </div>
                    </div>
                    
                    <a href="/codes" class="cta-button">Browse All Codes</a>
                </div>
                
                <div class="features">
                    <div class="feature-card">
                        <div class="feature-icon">🔍</div>
                        <h3>Smart Search</h3>
                        <p>Search by code, description, or any keyword to quickly find what you need</p>
                    </div>
                    <div class="feature-card">
                        <div class="feature-icon">🎯</div>
                        <h3>Advanced Filtering</h3>
                        <p>Filter by category, system, and severity level to narrow down results</p>
                    </div>
                    <div class="feature-card">
                        <div class="feature-icon">📊</div>
                        <h3>Easy Sorting</h3>
                        <p>Sort codes by any column to organize data the way you want</p>
                    </div>
                    <div class="feature-card">
                        <div class="feature-icon">🚗</div>
                        <h3>Complete Coverage</h3>
                        <p>Covers Powertrain, Body, Chassis, and Network diagnostic codes</p>
                    </div>
                </div>
                
                <div style="text-align: center; margin-top: 40px; color: #666;">
                    <p>Created by Abdulwahed | <a href="https://github.com/abdulwahed-sweden/odb-codes-to-json" style="color: #2a5298;">View on GitHub</a></p>
                </div>
            </div>
        </body>
        </html>
    "#)
}

#[get("/codes")]
async fn codes_handler() -> impl Responder {
    let codes_file_path = "odb_codes.json";
    
    match fs::read_to_string(codes_file_path) {
        Ok(data) => {
            match serde_json::from_str::<OdbDatabase>(&data) {
                Ok(odb_data) => {
                    let mut html = String::from(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>ODB Diagnostic Codes - Browse</title>
    <style>
        * {
            box-sizing: border-box;
        }
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            margin: 0;
            padding: 0;
            background: #f5f5f5;
        }
        .container {
            max-width: 1400px;
            margin: 0 auto;
            padding: 20px;
        }
        .header {
            background: linear-gradient(135deg, #1e3c72 0%, #2a5298 100%);
            color: white;
            padding: 30px;
            border-radius: 10px;
            margin-bottom: 30px;
            box-shadow: 0 4px 6px rgba(0,0,0,0.1);
        }
        h1 {
            margin: 0 0 10px 0;
            font-size: 2em;
        }
        .controls {
            background: white;
            padding: 20px;
            border-radius: 10px;
            margin-bottom: 20px;
            box-shadow: 0 2px 5px rgba(0,0,0,0.1);
        }
        .control-row {
            display: flex;
            gap: 15px;
            flex-wrap: wrap;
            margin-bottom: 15px;
        }
        .search-box {
            flex: 1;
            min-width: 300px;
        }
        input[type="text"] {
            width: 100%;
            padding: 10px 15px;
            border: 2px solid #ddd;
            border-radius: 5px;
            font-size: 16px;
            transition: border-color 0.3s;
        }
        input[type="text"]:focus {
            outline: none;
            border-color: #2a5298;
        }
        select {
            padding: 10px 15px;
            border: 2px solid #ddd;
            border-radius: 5px;
            font-size: 16px;
            background: white;
            cursor: pointer;
            transition: border-color 0.3s;
        }
        select:focus {
            outline: none;
            border-color: #2a5298;
        }
        .stats {
            display: flex;
            gap: 20px;
            margin-top: 15px;
            flex-wrap: wrap;
        }
        .stat-item {
            background: #f0f0f0;
            padding: 10px 20px;
            border-radius: 5px;
            font-size: 14px;
        }
        .table-container {
            background: white;
            border-radius: 10px;
            overflow: hidden;
            box-shadow: 0 2px 5px rgba(0,0,0,0.1);
        }
        table {
            width: 100%;
            border-collapse: collapse;
        }
        th {
            background: #2a5298;
            color: white;
            padding: 15px;
            text-align: left;
            cursor: pointer;
            user-select: none;
            position: sticky;
            top: 0;
            z-index: 10;
        }
        th:hover {
            background: #1e3c72;
        }
        th.sortable::after {
            content: ' ↕';
            opacity: 0.5;
        }
        th.sorted-asc::after {
            content: ' ↑';
            opacity: 1;
        }
        th.sorted-desc::after {
            content: ' ↓';
            opacity: 1;
        }
        td {
            padding: 12px 15px;
            border-bottom: 1px solid #eee;
        }
        tr:hover {
            background: #f8f8f8;
        }
        .code-badge {
            display: inline-block;
            padding: 4px 8px;
            border-radius: 3px;
            font-weight: bold;
            font-family: monospace;
        }
        .code-P { background: #e3f2fd; color: #1565c0; }
        .code-B { background: #f3e5f5; color: #6a1b9a; }
        .code-C { background: #e8f5e9; color: #2e7d32; }
        .code-U { background: #fff3e0; color: #ef6c00; }
        
        .severity-badge {
            display: inline-block;
            padding: 4px 12px;
            border-radius: 3px;
            font-size: 12px;
            font-weight: bold;
        }
        .severity-Info { background: #e1f5fe; color: #0277bd; }
        .severity-Warning { background: #fff9c4; color: #f57c00; }
        .severity-Critical { background: #ffebee; color: #c62828; }
        
        .back-link {
            display: inline-block;
            color: white;
            text-decoration: none;
            opacity: 0.8;
            transition: opacity 0.3s;
        }
        .back-link:hover {
            opacity: 1;
        }
        .no-results {
            text-align: center;
            padding: 40px;
            color: #666;
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <a href="/" class="back-link">← Back to Home</a>
            <h1>OBD Diagnostic Trouble Codes</h1>
            <p style="margin: 0; opacity: 0.9;">Browse, search, and filter all "#);
                    
                    html.push_str(&odb_data.metadata.total_codes.to_string());
                    html.push_str(r#" diagnostic codes</p>
        </div>
        
        <div class="controls">
            <div class="control-row">
                <div class="search-box">
                    <input type="text" id="searchInput" placeholder="Search by code or description..." onkeyup="filterTable()">
                </div>
                <select id="categoryFilter" onchange="filterTable()">
                    <option value="">All Categories</option>
                    <option value="Powertrain">Powertrain (P)</option>
                    <option value="Body">Body (B)</option>
                    <option value="Chassis">Chassis (C)</option>
                    <option value="Network">Network (U)</option>
                </select>
                <select id="systemFilter" onchange="filterTable()">
                    <option value="">All Systems</option>"#);
                    
                    // Add all unique systems
                    let mut systems: Vec<&str> = odb_data.diagnostic_codes.iter()
                        .map(|c| c.system.as_str())
                        .collect();
                    systems.sort();
                    systems.dedup();
                    
                    for system in systems {
                        html.push_str(&format!(r#"<option value="{}">{}</option>"#, system, system));
                    }
                    
                    html.push_str(r#"</select>
                <select id="severityFilter" onchange="filterTable()">
                    <option value="">All Severities</option>
                    <option value="Info">Info</option>
                    <option value="Warning">Warning</option>
                    <option value="Critical">Critical</option>
                </select>
            </div>
            <div class="stats">
                <div class="stat-item">
                    <strong>Total:</strong> <span id="totalCount">"#);
                    html.push_str(&odb_data.diagnostic_codes.len().to_string());
                    html.push_str(r#"</span>
                </div>
                <div class="stat-item">
                    <strong>Filtered:</strong> <span id="filteredCount">"#);
                    html.push_str(&odb_data.diagnostic_codes.len().to_string());
                    html.push_str(r#"</span>
                </div>
                <div class="stat-item">
                    <strong>P:</strong> "#);
                    html.push_str(&odb_data.metadata.code_breakdown.p_codes.to_string());
                    html.push_str(r#" | <strong>B:</strong> "#);
                    html.push_str(&odb_data.metadata.code_breakdown.b_codes.to_string());
                    html.push_str(r#" | <strong>C:</strong> "#);
                    html.push_str(&odb_data.metadata.code_breakdown.c_codes.to_string());
                    html.push_str(r#" | <strong>U:</strong> "#);
                    html.push_str(&odb_data.metadata.code_breakdown.u_codes.to_string());
                    html.push_str(r#"
                </div>
            </div>
        </div>
        
        <div class="table-container">
            <table id="codesTable">
                <thead>
                    <tr>
                        <th class="sortable" onclick="sortTable(0)">Code</th>
                        <th class="sortable" onclick="sortTable(1)">Description</th>
                        <th class="sortable" onclick="sortTable(2)">Category</th>
                        <th class="sortable" onclick="sortTable(3)">System</th>
                        <th class="sortable" onclick="sortTable(4)">Severity</th>
                    </tr>
                </thead>
                <tbody>"#);
                    
                    // Add all diagnostic codes
                    for code in &odb_data.diagnostic_codes {
                        let code_type = &code.code[0..1];
                        html.push_str(&format!(r#"
                    <tr>
                        <td><span class="code-badge code-{}">{}</span></td>
                        <td>{}</td>
                        <td>{}</td>
                        <td>{}</td>
                        <td><span class="severity-badge severity-{}">{}</span></td>
                    </tr>"#,
                            code_type, code.code,
                            code.description,
                            code.category,
                            code.system,
                            code.severity, code.severity
                        ));
                    }
                    
                    html.push_str(r#"
                </tbody>
            </table>
        </div>
    </div>
    
    <script>
        let currentSort = { column: -1, ascending: true };
        
        function filterTable() {
            const searchInput = document.getElementById('searchInput').value.toLowerCase();
            const categoryFilter = document.getElementById('categoryFilter').value;
            const systemFilter = document.getElementById('systemFilter').value;
            const severityFilter = document.getElementById('severityFilter').value;
            
            const table = document.getElementById('codesTable');
            const rows = table.getElementsByTagName('tbody')[0].getElementsByTagName('tr');
            let visibleCount = 0;
            
            for (let row of rows) {
                const code = row.cells[0].textContent.toLowerCase();
                const description = row.cells[1].textContent.toLowerCase();
                const category = row.cells[2].textContent;
                const system = row.cells[3].textContent;
                const severity = row.cells[4].textContent.trim();
                
                const matchesSearch = code.includes(searchInput) || description.includes(searchInput);
                const matchesCategory = !categoryFilter || category === categoryFilter;
                const matchesSystem = !systemFilter || system === systemFilter;
                const matchesSeverity = !severityFilter || severity === severityFilter;
                
                if (matchesSearch && matchesCategory && matchesSystem && matchesSeverity) {
                    row.style.display = '';
                    visibleCount++;
                } else {
                    row.style.display = 'none';
                }
            }
            
            document.getElementById('filteredCount').textContent = visibleCount;
        }
        
        function sortTable(column) {
            const table = document.getElementById('codesTable');
            const tbody = table.getElementsByTagName('tbody')[0];
            const rows = Array.from(tbody.getElementsByTagName('tr'));
            const headers = table.getElementsByTagName('th');
            
            // Remove sorted classes from all headers
            for (let header of headers) {
                header.classList.remove('sorted-asc', 'sorted-desc');
            }
            
            // Determine sort direction
            if (currentSort.column === column) {
                currentSort.ascending = !currentSort.ascending;
            } else {
                currentSort.column = column;
                currentSort.ascending = true;
            }
            
            // Add appropriate class to current header
            headers[column].classList.add(currentSort.ascending ? 'sorted-asc' : 'sorted-desc');
            
            // Sort rows
            rows.sort((a, b) => {
                let aVal = a.cells[column].textContent.trim();
                let bVal = b.cells[column].textContent.trim();
                
                // Handle numeric sorting for code column
                if (column === 0) {
                    // Extract number from code (e.g., P0001 -> 1)
                    aVal = parseInt(aVal.substring(1)) || 0;
                    bVal = parseInt(bVal.substring(1)) || 0;
                }
                
                if (aVal < bVal) return currentSort.ascending ? -1 : 1;
                if (aVal > bVal) return currentSort.ascending ? 1 : -1;
                return 0;
            });
            
            // Reattach sorted rows
            rows.forEach(row => tbody.appendChild(row));
        }
        
        // Initialize with code column sorted ascending
        window.onload = function() {
            sortTable(0);
        };
    </script>
</body>
</html>"#);
                    
                    HttpResponse::Ok().content_type("text/html").body(html)
                }
                Err(e) => HttpResponse::InternalServerError().body(format!(
                    r#"<h1>Error</h1>
                    <p>Error parsing odb_codes.json: {}</p>
                    <p>Make sure the JSON file is properly formatted.</p>
                    <p><a href='/'>Back to Home</a></p>"#, 
                    e
                )),
            }
        }
        Err(_) => HttpResponse::InternalServerError().body(
            r#"<h1>Error</h1>
            <p>Error reading odb_codes.json. Please ensure the file exists in the project root.</p>
            <p>The file should be named 'odb_codes.json' and placed in the same directory as Cargo.toml</p>
            <p><a href='/'>Back to Home</a></p>"#
        ),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting OBD Diagnostic Codes Server at http://127.0.0.1:8080");
    println!("Make sure 'odb_codes.json' exists in the project root directory!");
    println!("Access the application at:");
    println!("  - Home: http://127.0.0.1:8080/");
    println!("  - Browse Codes: http://127.0.0.1:8080/codes");
    
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(codes_handler)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}