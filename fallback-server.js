const http = require('http');
const fs = require('fs');
const path = require('path');
const { exec } = require('child_process');

const PORT = process.env.PORT || 8080;

// Create a simple HTTP server
const server = http.createServer((req, res) => {
  // Handle restart request
  if (req.url === '/restart') {
    res.writeHead(200, {'Content-Type': 'text/html'});
    res.end(`
      <!DOCTYPE html>
      <html>
      <head>
        <title>IT Learning Platform - Restarting</title>
        <style>
          body { font-family: Arial, sans-serif; max-width: 800px; margin: 0 auto; padding: 20px; }
          .message { background: #f8f8f8; padding: 15px; border-radius: 5px; }
          .spinner { 
            border: 4px solid #f3f3f3;
            border-top: 4px solid #3498db;
            border-radius: 50%;
            width: 30px;
            height: 30px;
            animation: spin 2s linear infinite;
            margin: 20px auto;
          }
          @keyframes spin {
            0% { transform: rotate(0deg); }
            100% { transform: rotate(360deg); }
          }
        </style>
        <meta http-equiv="refresh" content="5;url=/">
      </head>
      <body>
        <h1>IT Learning Platform</h1>
        <div class="message">
          <h2>Restarting Application...</h2>
          <div class="spinner"></div>
          <p>The application is being restarted. You will be redirected in 5 seconds...</p>
        </div>
      </body>
      </html>
    `);
    
    // Actually restart the application
    console.log('Restart requested. Stopping fallback server and restarting main application...');
    
    // Execute restart in a separate process so we can finish the HTTP response
    exec('/app/start.sh > /app/app_output.log 2>&1 &', (error) => {
      if (error) {
        console.error(`Error restarting application: ${error}`);
      }
      // Shutdown this server after 2 seconds
      setTimeout(() => process.exit(0), 2000);
    });
    
    return;
  }
  
  // For any other request, show the maintenance page
  res.writeHead(200, {'Content-Type': 'text/html'});
  
  // Read diagnostic data
  let diagnosticOutput = "No diagnostic data available";
  try {
    if (fs.existsSync('/app/diagnostic-output.txt')) {
      diagnosticOutput = fs.readFileSync('/app/diagnostic-output.txt', 'utf8');
    }
  } catch (e) {
    diagnosticOutput = `Error reading diagnostics: ${e.message}`;
  }
  
  // Read application logs if available
  let appLogs = "No application logs available";
  try {
    if (fs.existsSync('/app/app_output.log')) {
      // Get only the last 50 lines
      const logContent = fs.readFileSync('/app/app_output.log', 'utf8');
      appLogs = logContent.split('\n').slice(-50).join('\n');
    }
  } catch (e) {
    appLogs = `Error reading application logs: ${e.message}`;
  }
  
  // Simple HTML response
  res.end(`
    <!DOCTYPE html>
    <html>
    <head>
      <title>IT Learning Platform - Maintenance</title>
      <style>
        body { font-family: Arial, sans-serif; max-width: 800px; margin: 0 auto; padding: 20px; }
        pre { background: #f4f4f4; padding: 15px; overflow: auto; border-radius: 5px; }
        .error { color: #cc0000; }
        .actions { margin-top: 20px; }
        .btn { display: inline-block; padding: 10px 15px; background: #4a6fa5; color: white; 
               text-decoration: none; border-radius: 4px; margin-right: 10px; }
        .tab { overflow: hidden; border: 1px solid #ccc; background-color: #f1f1f1; }
        .tab button { background-color: inherit; float: left; border: none; outline: none;
                     cursor: pointer; padding: 14px 16px; transition: 0.3s; }
        .tab button:hover { background-color: #ddd; }
        .tab button.active { background-color: #ccc; }
        .tabcontent { display: none; padding: 6px 12px; border: 1px solid #ccc; border-top: none; }
        .visible { display: block; }
      </style>
      <script>
        function openTab(evt, tabName) {
          var i, tabcontent, tablinks;
          tabcontent = document.getElementsByClassName("tabcontent");
          for (i = 0; i < tabcontent.length; i++) {
            tabcontent[i].style.display = "none";
          }
          tablinks = document.getElementsByClassName("tablinks");
          for (i = 0; i < tablinks.length; i++) {
            tablinks[i].className = tablinks[i].className.replace(" active", "");
          }
          document.getElementById(tabName).style.display = "block";
          evt.currentTarget.className += " active";
        }
      </script>
    </head>
    <body>
      <h1>IT Learning Platform</h1>
      <div class="error">
        <h2>⚠️ Application is currently in maintenance mode</h2>
        <p>The main application server couldn't start properly. A fallback server is running instead.</p>
      </div>
      
      <div class="tab">
        <button class="tablinks active" onclick="openTab(event, 'Diagnostics')">Diagnostics</button>
        <button class="tablinks" onclick="openTab(event, 'AppLogs')">Application Logs</button>
      </div>
      
      <div id="Diagnostics" class="tabcontent visible">
        <h3>Diagnostic Information:</h3>
        <pre>${diagnosticOutput}</pre>
      </div>
      
      <div id="AppLogs" class="tabcontent">
        <h3>Application Logs:</h3>
        <pre>${appLogs}</pre>
      </div>
      
      <div class="actions">
        <a href="/restart" class="btn">Restart Application</a>
      </div>
    </body>
    </html>
  `);
});

server.listen(PORT, '0.0.0.0', () => {
  console.log(`Fallback server running on port ${PORT}`);
});
