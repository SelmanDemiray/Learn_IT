document.addEventListener('DOMContentLoaded', function() {
    // Dark mode toggle functionality with enhanced UI feedback
    const darkModeToggle = document.querySelector('.dark-mode-toggle');
    if (darkModeToggle) {
        const sunIcon = darkModeToggle.querySelector('.sun-icon');
        const moonIcon = darkModeToggle.querySelector('.moon-icon');
        
        darkModeToggle.addEventListener('click', function() {
            document.body.classList.toggle('dark-mode');
            
            // Update icon visibility based on dark mode state
            const isDarkMode = document.body.classList.contains('dark-mode');
            sunIcon.style.display = isDarkMode ? 'block' : 'none';
            moonIcon.style.display = isDarkMode ? 'none' : 'block';
            
            // Add animation class
            darkModeToggle.classList.add('animate-toggle');
            setTimeout(() => darkModeToggle.classList.remove('animate-toggle'), 500);
            
            // Save preference
            localStorage.setItem('darkMode', isDarkMode ? 'enabled' : 'disabled');
        });
        
        // Check for saved dark mode preference and set initial icon state
        const isDarkMode = localStorage.getItem('darkMode') === 'enabled';
        if (isDarkMode) {
            document.body.classList.add('dark-mode');
            sunIcon.style.display = 'block';
            moonIcon.style.display = 'none';
        } else {
            sunIcon.style.display = 'none';
            moonIcon.style.display = 'block';
        }
    }
    
    // Improved Mobile menu toggle
    const mobileMenuToggle = document.querySelector('.mobile-menu-toggle');
    if (mobileMenuToggle) {
        mobileMenuToggle.addEventListener('click', function() {
            const navContainer = document.querySelector('.nav-container');
            mobileMenuToggle.classList.toggle('active');
            navContainer.classList.toggle('active');
            
            // Toggle the mobile menu animation
            const spans = mobileMenuToggle.querySelectorAll('span');
            if (mobileMenuToggle.classList.contains('active')) {
                spans[0].style.transform = 'rotate(45deg) translate(5px, 5px)';
                spans[1].style.opacity = '0';
                spans[2].style.transform = 'rotate(-45deg) translate(5px, -5px)';
            } else {
                spans[0].style.transform = 'none';
                spans[1].style.opacity = '1';
                spans[2].style.transform = 'none';
            }
            
            // Prevent scrolling when menu is open
            document.body.classList.toggle('menu-open');
        });
    }
    
    // Improved dropdown handling for mobile
    const dropdowns = document.querySelectorAll('.dropdown');
    dropdowns.forEach(dropdown => {
        const link = dropdown.querySelector('a');
        link.addEventListener('click', function(e) {
            // Only handle dropdown toggle on mobile
            if (window.innerWidth <= 992) {
                e.preventDefault();
                dropdown.classList.toggle('active');
            }
        });
    });
    
    // Enhanced active navigation highlighting
    const currentPath = window.location.pathname;
    const navLinks = document.querySelectorAll('nav ul li a');
    
    navLinks.forEach(link => {
        const href = link.getAttribute('href');
        
        // Check if current path starts with the link's href
        if ((currentPath === href) || 
            (currentPath.startsWith(href) && href !== '/' && href !== '#') ||
            (currentPath.match(/^\/courses\/\d+/) && href === '/courses') ||
            (currentPath.match(/^\/lessons\/\w+\//) && href === '/lessons')) {
            // Find closest li and make it active
            const parentLi = link.closest('li');
            if (parentLi) parentLi.classList.add('active');
        }
    });
    
    // Enhanced search functionality with better typeahead
    const searchInput = document.querySelector('.search-input');
    if (searchInput) {
        // Create autocomplete container if it doesn't exist
        let autocompleteContainer = document.querySelector('.search-autocomplete');
        if (!autocompleteContainer) {
            autocompleteContainer = document.createElement('div');
            autocompleteContainer.className = 'search-autocomplete';
            searchInput.parentNode.appendChild(autocompleteContainer);
        }
        
        searchInput.addEventListener('focus', function() {
            this.parentNode.classList.add('search-focused');
        });
        
        searchInput.addEventListener('blur', function() {
            setTimeout(() => {
                this.parentNode.classList.remove('search-focused');
                autocompleteContainer.innerHTML = '';
            }, 200);
        });
        
        // Expanded search suggestions with categories
        const searchSuggestions = [
            { text: 'HTML Basics', category: 'Beginner' }, 
            { text: 'CSS Fundamentals', category: 'Beginner' }, 
            { text: 'JavaScript Introduction', category: 'Beginner' }, 
            { text: 'Python for Beginners', category: 'Beginner' },
            { text: 'Network Security', category: 'Intermediate' }, 
            { text: 'Cloud Computing', category: 'Intermediate' },
            { text: 'Data Science Basics', category: 'Intermediate' }, 
            { text: 'Machine Learning Fundamentals', category: 'Advanced' },
            { text: 'Operating Systems', category: 'Beginner' },
            { text: 'Programming Concepts', category: 'Intermediate' }
        ];
        
        searchInput.addEventListener('input', function() {
            const query = this.value.trim().toLowerCase();
            autocompleteContainer.innerHTML = '';
            
            if (query.length < 2) return;
            
            const matches = searchSuggestions.filter(item => 
                item.text.toLowerCase().includes(query)
            );
            
            if (matches.length === 0) {
                const noResults = document.createElement('div');
                noResults.className = 'autocomplete-no-results';
                noResults.textContent = 'No results found';
                autocompleteContainer.appendChild(noResults);
                return;
            }
            
            matches.slice(0, 6).forEach(match => {
                const div = document.createElement('div');
                div.className = 'autocomplete-item';
                
                const text = document.createElement('span');
                text.className = 'item-text';
                text.textContent = match.text;
                
                const category = document.createElement('span');
                category.className = 'item-category ' + match.category.toLowerCase();
                category.textContent = match.category;
                
                div.appendChild(text);
                div.appendChild(category);
                
                div.addEventListener('click', () => {
                    searchInput.value = match.text;
                    window.location.href = `/search?q=${encodeURIComponent(match.text)}`;
                });
                autocompleteContainer.appendChild(div);
            });
        });
        
        searchInput.addEventListener('keypress', function(e) {
            if (e.key === 'Enter') {
                const query = this.value.trim();
                if (query) {
                    window.location.href = `/search?q=${encodeURIComponent(query)}`;
                }
            }
        });
    }
    
    // Add smooth scroll behavior for anchor links
    document.querySelectorAll('a[href^="#"]:not([href="#"])').forEach(anchor => {
        anchor.addEventListener('click', function(e) {
            e.preventDefault();
            const targetId = this.getAttribute('href').substring(1);
            const targetElement = document.getElementById(targetId);
            
            if (targetElement) {
                targetElement.scrollIntoView({ 
                    behavior: 'smooth',
                    block: 'start'
                });
            }
        });
    });
    
    // Add intersection observer for element animations
    const animateElements = document.querySelectorAll('.feature-card, .level-card, .course-card, .section-header, .cta-container');
    if (animateElements.length > 0) {
        const observer = new IntersectionObserver((entries, observer) => {
            entries.forEach(entry => {
                if (entry.isIntersecting) {
                    entry.target.classList.add('animate');
                    observer.unobserve(entry.target);
                }
            });
        }, { threshold: 0.1, rootMargin: '0px 0px -50px 0px' });
        
        animateElements.forEach((el, index) => {
            // Add base animation-delay class
            el.style.animationDelay = `${index * 0.1}s`;
            observer.observe(el);
        });
    }
    
    // Initialize progress bars with improved animation
    initProgressBars();
    
    // Setup code copy functionality
    setupCodeCopy();
});

// Enhanced progress bars with animation
function initProgressBars() {
    const progressBars = document.querySelectorAll('.progress-bar');
    
    progressBars.forEach(bar => {
        const progress = bar.querySelector('.progress');
        if (!progress) return;
        
        const targetWidth = progress.getAttribute('data-target-width') || progress.style.width || '0%';
        const progressText = bar.parentElement.querySelector('.progress-text');
        
        // Start at 0
        progress.style.width = '0%';
        progress.style.opacity = '0.3';
        
        // Get numeric percentage
        const percentage = parseInt(targetWidth);
        
        // Update progress text if available
        if (progressText && !isNaN(percentage)) {
            if (percentage === 0) {
                progressText.textContent = 'Not started';
            } else if (percentage === 100) {
                progressText.textContent = 'Completed';
            } else {
                progressText.textContent = `${percentage}% Complete`;
            }
        }
        
        // Animate to target width with staggered delay for multiple bars
        setTimeout(() => {
            progress.style.transition = 'width 1s ease-in-out, opacity 0.5s ease-in-out';
            progress.style.width = targetWidth;
            progress.style.opacity = '1';
        }, Math.random() * 300 + 100); // Stagger between 100-400ms
    });
}

// Improved course progress tracking
function updateProgress(courseId, lessonId, completed) {
    // This would normally send an AJAX request to update progress
    console.log(`Progress update: Course ${courseId}, Lesson ${lessonId}, Completed: ${completed}`);
    
    return fetch('/api/progress', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({
            courseId,
            lessonId,
            completed
        })
    })
    .then(response => {
        if (!response.ok) {
            throw new Error('Network response was not ok');
        }
        return response.json();
    })
    .then(data => {
        // Update UI with new progress data
        const progressBar = document.querySelector(`.course-${courseId} .progress`);
        if (progressBar) {
            progressBar.style.width = `${data.progress}%`;
            
            // Update text based on completion status
            const progressText = progressBar.parentElement.nextElementSibling;
            if (progressText) {
                if (data.progress === 0) {
                    progressText.textContent = 'Not started';
                } else if (data.progress === 100) {
                    progressText.textContent = 'Completed';
                } else {
                    progressText.textContent = `${data.progress}% Complete`;
                }
            }
        }
        
        // Return data for further processing
        return data;
    })
    .catch(error => {
        console.error('Error updating progress:', error);
        
        // For demo, update UI directly
        const progressBar = document.querySelector(`.course-${courseId} .progress`);
        if (progressBar) {
            const currentWidth = parseInt(progressBar.style.width) || 0;
            const lessonValue = 20; // Assuming each lesson is worth 20%
            const newWidth = Math.max(0, Math.min(100, completed ? currentWidth + lessonValue : currentWidth - lessonValue));
            
            // Animate the progress change
            progressBar.style.transition = 'width 0.5s ease-in-out';
            progressBar.style.width = `${newWidth}%`;
            
            // Update text
            const progressText = progressBar.parentElement.nextElementSibling;
            if (progressText) {
                if (newWidth === 0) {
                    progressText.textContent = 'Not started';
                } else if (newWidth === 100) {
                    progressText.textContent = 'Completed';
                } else {
                    progressText.textContent = `${newWidth}% Complete`;
                }
            }
        }
    });
}

// Improved code snippet copy functionality
function setupCodeCopy() {
    document.querySelectorAll('pre').forEach(block => {
        // Only add button if it doesn't already exist
        if (!block.querySelector('.copy-button')) {
            const button = document.createElement('button');
            button.className = 'copy-button';
            button.innerHTML = '<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path></svg>';
            button.setAttribute('aria-label', 'Copy code');
            
            // Create a tooltip element
            const tooltip = document.createElement('span');
            tooltip.className = 'copy-tooltip';
            tooltip.textContent = 'Copy';
            button.appendChild(tooltip);
            
            block.appendChild(button);
            
            button.addEventListener('click', function() {
                const code = block.querySelector('code') ? 
                    block.querySelector('code').textContent : 
                    block.textContent;
                
                navigator.clipboard.writeText(code).then(() => {
                    tooltip.textContent = 'Copied!';
                    button.classList.add('copied');
                    
                    setTimeout(() => {
                        tooltip.textContent = 'Copy';
                        button.classList.remove('copied');
                    }, 2000);
                }).catch(err => {
                    tooltip.textContent = 'Failed!';
                    console.error('Could not copy text: ', err);
                    
                    setTimeout(() => {
                        tooltip.textContent = 'Copy';
                    }, 2000);
                });
            });
        }
    });
}

// Add smooth page transitions
window.addEventListener('beforeunload', function() {
    document.body.classList.add('page-transition');
});
