document.addEventListener('DOMContentLoaded', function() {
    // Dark mode toggle functionality
    const darkModeToggle = document.querySelector('.dark-mode-toggle');
    if (darkModeToggle) {
        darkModeToggle.addEventListener('click', function() {
            document.body.classList.toggle('dark-mode');
            
            // Save preference
            if (document.body.classList.contains('dark-mode')) {
                localStorage.setItem('darkMode', 'enabled');
            } else {
                localStorage.setItem('darkMode', 'disabled');
            }
        });
        
        // Check for saved dark mode preference
        if (localStorage.getItem('darkMode') === 'enabled') {
            document.body.classList.add('dark-mode');
        }
    }
    
    // Active navigation highlighting
    const currentPath = window.location.pathname;
    const navLinks = document.querySelectorAll('nav ul li a');
    
    navLinks.forEach(link => {
        if (currentPath === link.getAttribute('href')) {
            link.parentElement.classList.add('active');
        } else if (currentPath.startsWith(link.getAttribute('href')) && link.getAttribute('href') !== '/') {
            link.parentElement.classList.add('active');
        }
    });
    
    // Initialize search functionality
    const searchInput = document.querySelector('.search-input');
    if (searchInput) {
        searchInput.addEventListener('keypress', function(e) {
            if (e.key === 'Enter') {
                const query = this.value.trim();
                if (query) {
                    window.location.href = `/search?q=${encodeURIComponent(query)}`;
                }
            }
        });
    }
    
    // Add animation to page elements
    const animateElements = document.querySelectorAll('.feature-card, .level-card, .course-card');
    if (animateElements.length > 0) {
        const observer = new IntersectionObserver((entries, observer) => {
            entries.forEach(entry => {
                if (entry.isIntersecting) {
                    entry.target.classList.add('animate');
                    observer.unobserve(entry.target);
                }
            });
        }, { threshold: 0.1 });
        
        animateElements.forEach(el => {
            observer.observe(el);
        });
    }
});

// Helper functions for course progress
function updateProgress(courseId, lessonId, completed) {
    // This would normally send an AJAX request to update progress in the backend
    console.log(`Progress update: Course ${courseId}, Lesson ${lessonId}, Completed: ${completed}`);
    
    // For demo purposes, update UI directly
    const progressBar = document.querySelector(`.course-${courseId} .progress`);
    if (progressBar) {
        const width = parseInt(progressBar.style.width) || 0;
        const newWidth = completed ? width + 20 : width - 20; // Assuming each lesson is worth 20%
        progressBar.style.width = `${Math.max(0, Math.min(100, newWidth))}%`;
    }
}

// Code snippet copy functionality
function setupCodeCopy() {
    document.querySelectorAll('pre').forEach(block => {
        const button = document.createElement('button');
        button.className = 'copy-button';
        button.textContent = 'Copy';
        block.appendChild(button);
        
        button.addEventListener('click', function() {
            const code = block.querySelector('code').textContent;
            navigator.clipboard.writeText(code).then(() => {
                button.textContent = 'Copied!';
                setTimeout(() => {
                    button.textContent = 'Copy';
                }, 2000);
            }).catch(err => {
                console.error('Could not copy text: ', err);
            });
        });
    });
}

// Call this function after content loads
document.addEventListener('DOMContentLoaded', setupCodeCopy);
