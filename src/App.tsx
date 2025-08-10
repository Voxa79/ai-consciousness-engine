import React, { Suspense, lazy } from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import { QueryClient, QueryClientProvider } from 'react-query';
import { Toaster } from 'react-hot-toast';
import { ErrorBoundary } from './components/ErrorBoundary';
import { LoadingSpinner } from './components/LoadingSpinner';
import { Navigation } from './components/Navigation';
import { ConsciousnessProvider } from './contexts/ConsciousnessContext';
import './App.css';

// Lazy load components for better performance
const HomePage = lazy(() => import('./pages/HomePage'));
const ConsciousnessPage = lazy(() => import('./pages/ConsciousnessPage'));
const NeuralInterfacePage = lazy(() => import('./pages/NeuralInterfacePage'));
const QuantumPage = lazy(() => import('./pages/QuantumPage'));
const NanotechnologyPage = lazy(() => import('./pages/NanotechnologyPage'));
const SpaceNetworkPage = lazy(() => import('./pages/SpaceNetworkPage'));
const TranscendencePage = lazy(() => import('./pages/TranscendencePage'));
const SingularityPage = lazy(() => import('./pages/SingularityPage'));
const DashboardPage = lazy(() => import('./pages/DashboardPage'));
const AnalyticsPage = lazy(() => import('./pages/AnalyticsPage'));
const SettingsPage = lazy(() => import('./pages/SettingsPage'));

// Create a client
const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      retry: 3,
      staleTime: 5 * 60 * 1000, // 5 minutes
      cacheTime: 10 * 60 * 1000, // 10 minutes
    },
  },
});

function App() {
  return (
    <ErrorBoundary>
      <QueryClientProvider client={queryClient}>
        <ConsciousnessProvider>
          <Router>
            <div className="min-h-screen bg-gradient-to-br from-slate-900 via-purple-900 to-slate-900">
              {/* Background Effects */}
              <div className="fixed inset-0 bg-[url('/assets/neural-network.svg')] opacity-5"></div>
              <div className="fixed inset-0 bg-gradient-to-r from-blue-500/10 via-purple-500/10 to-pink-500/10 animate-pulse"></div>
              
              {/* Navigation */}
              <Navigation />
              
              {/* Main Content */}
              <main className="relative z-10">
                <Suspense fallback={
                  <div className="flex items-center justify-center min-h-screen">
                    <LoadingSpinner size="large" />
                  </div>
                }>
                  <Routes>
                    {/* Home */}
                    <Route path="/" element={<HomePage />} />
                    
                    {/* Core Consciousness */}
                    <Route path="/consciousness" element={<ConsciousnessPage />} />
                    <Route path="/dashboard" element={<DashboardPage />} />
                    
                    {/* Neural Interfaces */}
                    <Route path="/neural" element={<NeuralInterfacePage />} />
                    <Route path="/neural/calibration" element={<NeuralInterfacePage />} />
                    <Route path="/neural/feedback" element={<NeuralInterfacePage />} />
                    
                    {/* Quantum Computing */}
                    <Route path="/quantum" element={<QuantumPage />} />
                    <Route path="/quantum/circuits" element={<QuantumPage />} />
                    <Route path="/quantum/algorithms" element={<QuantumPage />} />
                    
                    {/* Nanotechnology */}
                    <Route path="/nano" element={<NanotechnologyPage />} />
                    <Route path="/nano/molecular" element={<NanotechnologyPage />} />
                    <Route path="/nano/assembly" element={<NanotechnologyPage />} />
                    
                    {/* Space Network */}
                    <Route path="/space" element={<SpaceNetworkPage />} />
                    <Route path="/space/missions" element={<SpaceNetworkPage />} />
                    <Route path="/space/colonies" element={<SpaceNetworkPage />} />
                    
                    {/* Transcendence */}
                    <Route path="/transcendence" element={<TranscendencePage />} />
                    <Route path="/singularity" element={<SingularityPage />} />
                    
                    {/* Analytics & Settings */}
                    <Route path="/analytics" element={<AnalyticsPage />} />
                    <Route path="/settings" element={<SettingsPage />} />
                    
                    {/* Catch all */}
                    <Route path="*" element={<HomePage />} />
                  </Routes>
                </Suspense>
              </main>
              
              {/* Toast Notifications */}
              <Toaster
                position="top-right"
                toastOptions={{
                  duration: 4000,
                  style: {
                    background: 'rgba(15, 23, 42, 0.95)',
                    color: '#f1f5f9',
                    border: '1px solid rgba(139, 92, 246, 0.3)',
                    backdropFilter: 'blur(10px)',
                  },
                  success: {
                    iconTheme: {
                      primary: '#10b981',
                      secondary: '#f1f5f9',
                    },
                  },
                  error: {
                    iconTheme: {
                      primary: '#ef4444',
                      secondary: '#f1f5f9',
                    },
                  },
                }}
              />
              
              {/* Consciousness Particles Effect */}
              <div className="fixed inset-0 pointer-events-none">
                {Array.from({ length: 50 }).map((_, i) => (
                  <div
                    key={i}
                    className="absolute w-1 h-1 bg-purple-400 rounded-full opacity-30 animate-float"
                    style={{
                      left: `${Math.random() * 100}%`,
                      top: `${Math.random() * 100}%`,
                      animationDelay: `${Math.random() * 10}s`,
                      animationDuration: `${10 + Math.random() * 20}s`,
                    }}
                  />
                ))}
              </div>
            </div>
          </Router>
        </ConsciousnessProvider>
      </QueryClientProvider>
    </ErrorBoundary>
  );
}

export default App;
