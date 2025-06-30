use crate::monitoring::Monitor;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

pub struct WebServer {
    monitor: Arc<Monitor>,
    port: u16,
}

impl WebServer {
    pub fn new(monitor: Arc<Monitor>, port: u16) -> Self {
        Self { monitor, port }
    }

    pub async fn start(&self) -> anyhow::Result<()> {
        let addr = format!("0.0.0.0:{}", self.port);
        let listener = TcpListener::bind(&addr).await?;
        println!("Dashboard server running at http://{}", addr);

        let monitor = self.monitor.clone();

        tokio::spawn(async move {
            loop {
                let (mut socket, _) = listener.accept().await.unwrap();
                let monitor = monitor.clone();

                tokio::spawn(async move {
                    let mut buffer = [0; 1024];
                    let n = socket.read(&mut buffer).await.unwrap();
                    let request = String::from_utf8_lossy(&buffer[..n]);

                    let response = if request.starts_with("GET / ")
                        || request.starts_with("GET /index.html")
                    {
                        Self::serve_dashboard()
                    } else if request.starts_with("GET /api/actions") {
                        Self::serve_actions(&monitor)
                    } else if request.starts_with("GET /api/llm-calls") {
                        Self::serve_llm_calls(&monitor)
                    } else {
                        Self::not_found()
                    };

                    socket.write_all(response.as_bytes()).await.unwrap();
                    socket.flush().await.unwrap();
                });
            }
        });

        Ok(())
    }

    fn serve_dashboard() -> String {
        let html = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Agent Monitoring Dashboard</title>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            background-color: #f5f5f5;
            color: #333;
        }
        
        .container {
            max-width: 1400px;
            margin: 0 auto;
            padding: 20px;
        }
        
        h1 {
            text-align: center;
            margin-bottom: 30px;
            color: #2c3e50;
        }
        
        .tabs {
            display: flex;
            gap: 10px;
            margin-bottom: 20px;
            border-bottom: 2px solid #ddd;
        }
        
        .tab {
            padding: 10px 20px;
            background: none;
            border: none;
            font-size: 16px;
            cursor: pointer;
            color: #666;
            transition: all 0.3s;
            border-bottom: 3px solid transparent;
        }
        
        .tab:hover {
            color: #333;
        }
        
        .tab.active {
            color: #2c3e50;
            border-bottom-color: #3498db;
        }
        
        .tab-content {
            display: none;
            animation: fadeIn 0.3s;
        }
        
        .tab-content.active {
            display: block;
        }
        
        @keyframes fadeIn {
            from { opacity: 0; }
            to { opacity: 1; }
        }
        
        .log-entry {
            background: white;
            border-radius: 8px;
            padding: 15px;
            margin-bottom: 10px;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
            transition: transform 0.2s;
        }
        
        .log-entry:hover {
            transform: translateY(-2px);
            box-shadow: 0 4px 8px rgba(0,0,0,0.15);
        }
        
        .timestamp {
            color: #7f8c8d;
            font-size: 14px;
            margin-bottom: 5px;
        }
        
        .action-name {
            font-weight: bold;
            color: #2c3e50;
            margin-bottom: 5px;
        }
        
        .result {
            background: #ecf0f1;
            padding: 8px;
            border-radius: 4px;
            font-family: 'Courier New', monospace;
            font-size: 14px;
            margin-top: 5px;
            max-height: 200px;
            overflow-y: auto;
        }
        
        .duration {
            color: #27ae60;
            font-size: 14px;
            margin-top: 5px;
        }
        
        .prompt-section {
            margin-top: 10px;
        }
        
        .prompt-label {
            font-weight: bold;
            color: #34495e;
            margin-bottom: 5px;
        }
        
        .prompt-content {
            background: #f8f9fa;
            padding: 10px;
            border-radius: 4px;
            font-size: 14px;
            white-space: pre-wrap;
            word-wrap: break-word;
            position: relative;
        }
        
        .prompt-content.collapsed {
            max-height: 100px;
            overflow: hidden;
        }
        
        .prompt-content.collapsed::after {
            content: '';
            position: absolute;
            bottom: 0;
            left: 0;
            right: 0;
            height: 30px;
            background: linear-gradient(to bottom, transparent, #f8f9fa);
        }
        
        .expand-toggle {
            background: #3498db;
            color: white;
            border: none;
            padding: 4px 12px;
            border-radius: 4px;
            cursor: pointer;
            font-size: 12px;
            margin-top: 8px;
            transition: background 0.2s;
        }
        
        .expand-toggle:hover {
            background: #2980b9;
        }
        
        .model-info {
            color: #9b59b6;
            font-size: 14px;
            margin-top: 5px;
        }
        
        .refresh-btn {
            position: fixed;
            bottom: 20px;
            right: 20px;
            background: #3498db;
            color: white;
            border: none;
            padding: 12px 24px;
            border-radius: 25px;
            cursor: pointer;
            font-size: 16px;
            box-shadow: 0 4px 6px rgba(0,0,0,0.1);
            transition: all 0.3s;
        }
        
        .refresh-btn:hover {
            background: #2980b9;
            transform: translateY(-2px);
            box-shadow: 0 6px 8px rgba(0,0,0,0.15);
        }
        
        .loading {
            text-align: center;
            padding: 40px;
            color: #7f8c8d;
        }
        
        .error {
            background: #e74c3c;
            color: white;
            padding: 15px;
            border-radius: 8px;
            margin: 20px 0;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>Agent Monitoring Dashboard</h1>
        
        <div class="tabs">
            <button class="tab active" onclick="showTab('actions')">Action History</button>
            <button class="tab" onclick="showTab('llm')">LLM Call History</button>
        </div>
        
        <div id="actions" class="tab-content active">
            <div class="loading">Loading action history...</div>
        </div>
        
        <div id="llm" class="tab-content">
            <div class="loading">Loading LLM call history...</div>
        </div>
    </div>
    
    <button class="refresh-btn" onclick="refreshData()">Refresh</button>
    
    <script>
        let currentTab = 'actions';
        const expandedStates = new Set(); // Store IDs of expanded elements

        function showTab(tab) {
            currentTab = tab;
            document.querySelectorAll('.tab').forEach(t => t.classList.remove('active'));
            document.querySelectorAll('.tab-content').forEach(c => c.classList.remove('active'));
            
            event.target.classList.add('active');
            document.getElementById(tab).classList.add('active');
            
            // Clear expanded states when switching tabs
            expandedStates.clear();

            if (tab === 'actions') {
                loadActions();
            } else {
                loadLLMCalls();
            }
        }
        
        async function loadActions() {
            try {
                const response = await fetch('/api/actions');
                const data = await response.json();
                
                const container = document.getElementById('actions');
                if (data.length === 0) {
                    container.innerHTML = '<div class="loading">No actions recorded yet.</div>';
                    return;
                }
                
                container.innerHTML = data.map((action, index) => {
                    const resultId = `action-result-${index}`;
                    const isExpanded = expandedStates.has(resultId) ? '' : 'collapsed';
                    const buttonText = expandedStates.has(resultId) ? 'Show Less' : 'Show More';

                    return `
                        <div class="log-entry">
                            <div class="timestamp">${new Date(action.timestamp).toLocaleString()}</div>
                            <div class="action-name">${getActionName(action.action)}</div>
                            ${action.result ? `
                                <div class="prompt-section">
                                    <div class="prompt-label">Result:</div>
                                    <div id="${resultId}" class="prompt-content ${isExpanded}">${escapeHtml(action.result)}</div>
                                    ${action.result.length > 200 ? `<button class="expand-toggle" onclick="toggleExpand('${resultId}', this)">${buttonText}</button>` : ''}
                                </div>
                            ` : ''}
                            <div class="duration">Duration: ${action.duration_ms}ms</div>
                        </div>
                    `;
                }).join('');
            } catch (error) {
                document.getElementById('actions').innerHTML = 
                    '<div class="error">Error loading actions: ' + error.message + '</div>';
            }
        }
        
        async function loadLLMCalls() {
            try {
                const response = await fetch('/api/llm-calls');
                const data = await response.json();
                
                const container = document.getElementById('llm');
                if (data.length === 0) {
                    container.innerHTML = '<div class="loading">No LLM calls recorded yet.</div>';
                    return;
                }
                
                container.innerHTML = data.map((call, index) => {
                    const systemPromptId = `system-${index}`;
                    const userPromptId = `user-${index}`;
                    const responseId = `response-${index}`;

                    const isSystemExpanded = expandedStates.has(systemPromptId) ? '' : 'collapsed';
                    const isUserExpanded = expandedStates.has(userPromptId) ? '' : 'collapsed';
                    const isResponseExpanded = expandedStates.has(responseId) ? '' : 'collapsed';

                    const systemButtonText = expandedStates.has(systemPromptId) ? 'Show Less' : 'Show More';
                    const userButtonText = expandedStates.has(userPromptId) ? 'Show Less' : 'Show More';
                    const responseButtonText = expandedStates.has(responseId) ? 'Show Less' : 'Show More';
                    
                    return `
                        <div class="log-entry">
                            <div class="timestamp">${new Date(call.timestamp).toLocaleString()}</div>
                            <div class="model-info">Model: ${call.model}</div>
                            <div class="prompt-section">
                                <div class="prompt-label">System Prompt:</div>
                                <div id="${systemPromptId}" class="prompt-content ${isSystemExpanded}">${escapeHtml(call.system_prompt)}</div>
                                ${call.system_prompt.length > 200 ? `<button class="expand-toggle" onclick="toggleExpand('${systemPromptId}', this)">${systemButtonText}</button>` : ''}
                            </div>
                            <div class="prompt-section">
                                <div class="prompt-label">User Prompt:</div>
                                <div id="${userPromptId}" class="prompt-content ${isUserExpanded}">${escapeHtml(call.user_prompt)}</div>
                                ${call.user_prompt.length > 200 ? `<button class="expand-toggle" onclick="toggleExpand('${userPromptId}', this)">${userButtonText}</button>` : ''}
                            </div>
                            <div class="prompt-section">
                                <div class="prompt-label">Response:</div>
                                <div id="${responseId}" class="prompt-content ${isResponseExpanded}">${escapeHtml(call.response)}</div>
                                ${call.response.length > 200 ? `<button class="expand-toggle" onclick="toggleExpand('${responseId}', this)">${responseButtonText}</button>` : ''}
                            </div>
                            <div class="duration">Duration: ${call.duration_ms}ms</div>
                        </div>
                    `;
                }).join('');
            } catch (error) {
                document.getElementById('llm').innerHTML = 
                    '<div class="error">Error loading LLM calls: ' + error.message + '</div>';
            }
        }
        
        function getActionName(action) {
            if (typeof action === 'string') {
                return action;
            }
            return Object.keys(action)[0];
        }
        
        function escapeHtml(text) {
            const div = document.createElement('div');
            div.textContent = text;
            return div.innerHTML;
        }
        
        function toggleExpand(elementId, button) {
            const element = document.getElementById(elementId);
            if (element.classList.contains('collapsed')) {
                element.classList.remove('collapsed');
                button.textContent = 'Show Less';
                expandedStates.add(elementId);
            } else {
                element.classList.add('collapsed');
                button.textContent = 'Show More';
                expandedStates.delete(elementId);
            }
        }
        
        function refreshData() {
            if (currentTab === 'actions') {
                loadActions();
            } else {
                loadLLMCalls();
            }
        }
        
        // Initial load
        loadActions();
        
        // Auto-refresh every 5 seconds
        setInterval(refreshData, 25000);
    </script>
</body>
</html>"#;

        format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
            html.len(),
            html
        )
    }

    fn serve_actions(monitor: &Monitor) -> String {
        let actions = monitor.get_action_logs();
        let json = serde_json::to_string(&actions).unwrap_or_else(|_| "[]".to_string());

        format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            json.len(),
            json
        )
    }

    fn serve_llm_calls(monitor: &Monitor) -> String {
        let llm_calls = monitor.get_llm_call_logs();
        let json = serde_json::to_string(&llm_calls).unwrap_or_else(|_| "[]".to_string());

        format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            json.len(),
            json
        )
    }

    fn not_found() -> String {
        let body = "404 Not Found";
        format!(
            "HTTP/1.1 404 Not Found\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        )
    }
}
