import React from 'react';
import { createRoot } from 'react-dom/client';
import App from './App';
import './index.css';

function setupDarkMode() {
  const dark = localStorage.getItem('theme') === 'dark' ||
    (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches);
  if (dark) document.documentElement.classList.add('dark');
  else document.documentElement.classList.remove('dark');
}
setupDarkMode();

createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
