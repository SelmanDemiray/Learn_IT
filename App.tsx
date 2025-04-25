import React, { useState, useEffect } from 'react';
import { BrowserRouter, Routes, Route } from 'react-router-dom';
import Navbar from './components/Navbar';
import Home from './pages/Home';
import Lessons from './pages/Lessons';
import LessonDetail from './pages/LessonDetail';
import Progress from './pages/Progress';
import Dictionary from './components/Dictionary';
import Skills from './pages/Skills';

function App() {
  const [dictOpen, setDictOpen] = useState(false);
  const [dark, setDark] = useState(() =>
    localStorage.getItem('theme') === 'dark' ||
    (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches)
  );

  useEffect(() => {
    const openHandler = () => setDictOpen(true);
    window.addEventListener('openDictionary', openHandler);
    return () => window.removeEventListener('openDictionary', openHandler);
  }, []);

  useEffect(() => {
    if (dark) {
      document.documentElement.classList.add('dark');
      localStorage.setItem('theme', 'dark');
    } else {
      document.documentElement.classList.remove('dark');
      localStorage.setItem('theme', 'light');
    }
  }, [dark]);

  return (
    <BrowserRouter>
      <Navbar
        onDictionary={() => setDictOpen(true)}
        dark={dark}
        onToggleDark={() => setDark(d => !d)}
      />
      <div className="container mx-auto mt-8 px-4">
        <Routes>
          <Route path="/" element={<Home />} />
          <Route path="/lessons" element={<Lessons />} />
          <Route path="/lessons/:id" element={<LessonDetail />} />
          <Route path="/progress" element={<Progress />} />
          <Route path="/skills" element={<Skills />} />
        </Routes>
      </div>
      <Dictionary open={dictOpen} onClose={() => setDictOpen(false)} />
    </BrowserRouter>
  );
}

export default App;
