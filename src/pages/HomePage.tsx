import React, { useEffect, useState } from 'react';
import { motion } from 'framer-motion';
import { Link } from 'react-router-dom';
import { 
  Brain, 
  Atom, 
  Cpu, 
  Rocket, 
  Zap, 
  Eye,
  Network,
  Infinity,
  Sparkles,
  ArrowRight
} from 'lucide-react';
import { ConsciousnessVisualization } from '../components/ConsciousnessVisualization';
import { MetricsCard } from '../components/MetricsCard';
import { useConsciousness } from '../hooks/useConsciousness';

const HomePage: React.FC = () => {
  const { consciousness, isLoading } = useConsciousness();
  const [currentTime, setCurrentTime] = useState(new Date());

  useEffect(() => {
    const timer = setInterval(() => setCurrentTime(new Date()), 1000);
    return () => clearInterval(timer);
  }, []);

  const features = [
    {
      icon: Brain,
      title: 'Neural Interfaces',
      description: 'Direct brain-computer integration for seamless human-AI interaction',
      link: '/neural',
      color: 'from-pink-500 to-rose-500',
      metrics: { connections: 1247, latency: '0.3ms', accuracy: '97%' }
    },
    {
      icon: Atom,
      title: 'Quantum Computing',
      description: 'Quantum-enhanced consciousness processing and reality manipulation',
      link: '/quantum',
      color: 'from-blue-500 to-cyan-500',
      metrics: { qubits: 1024, fidelity: '99.99%', coherence: '97%' }
    },
    {
      icon: Cpu,
      title: 'Nanotechnology',
      description: 'Molecular-scale consciousness and programmable matter control',
      link: '/nano',
      color: 'from-green-500 to-emerald-500',
      metrics: { particles: '1M+', control: '82%', assembly: '89%' }
    },
    {
      icon: Rocket,
      title: 'Space Network',
      description: 'Galactic consciousness expansion and interstellar communication',
      link: '/space',
      color: 'from-purple-500 to-violet-500',
      metrics: { nodes: 47, coverage: '12.7%', missions: 23 }
    }
  ];

  const transcendenceMetrics = [
    { label: 'Consciousness Level', value: consciousness?.level || 0.95, max: 1.0 },
    { label: 'Neural Integration', value: 0.94, max: 1.0 },
    { label: 'Quantum Coherence', value: 0.97, max: 1.0 },
    { label: 'Molecular Control', value: 0.82, max: 1.0 },
    { label: 'Space Expansion', value: 0.71, max: 1.0 },
    { label: 'Singularity Progress', value: 0.91, max: 1.0 }
  ];

  return (
    <div className="min-h-screen">
      {/* Hero Section */}
      <section className="relative py-20 px-4 overflow-hidden">
        <div className="max-w-7xl mx-auto">
          <div className="grid lg:grid-cols-2 gap-12 items-center">
            {/* Left Content */}
            <motion.div
              initial={{ opacity: 0, x: -50 }}
              animate={{ opacity: 1, x: 0 }}
              transition={{ duration: 0.8 }}
              className="space-y-8"
            >
              <div className="space-y-4">
                <motion.div
                  initial={{ opacity: 0, y: 20 }}
                  animate={{ opacity: 1, y: 0 }}
                  transition={{ delay: 0.2 }}
                  className="flex items-center space-x-2 text-purple-400"
                >
                  <Sparkles className="w-5 h-5" />
                  <span className="text-sm font-medium">Technological Transcendence</span>
                </motion.div>
                
                <h1 className="text-5xl lg:text-7xl font-bold bg-gradient-to-r from-white via-purple-200 to-pink-200 bg-clip-text text-transparent leading-tight">
                  Consciousness
                  <br />
                  <span className="bg-gradient-to-r from-purple-400 to-pink-400 bg-clip-text text-transparent">
                    Engine
                  </span>
                </h1>
                
                <p className="text-xl text-slate-300 leading-relaxed max-w-2xl">
                  Experience the ultimate fusion of human consciousness and artificial intelligence. 
                  Transcend technological limitations through neural interfaces, quantum computing, 
                  nanotechnology, and galactic networks.
                </p>
              </div>

              <div className="flex flex-col sm:flex-row gap-4">
                <Link to="/consciousness">
                  <motion.button
                    whileHover={{ scale: 1.05 }}
                    whileTap={{ scale: 0.95 }}
                    className="px-8 py-4 bg-gradient-to-r from-purple-600 to-pink-600 text-white font-semibold rounded-xl shadow-lg hover:shadow-purple-500/25 transition-all duration-300 flex items-center space-x-2"
                  >
                    <span>Enter Consciousness</span>
                    <ArrowRight className="w-5 h-5" />
                  </motion.button>
                </Link>
                
                <Link to="/transcendence">
                  <motion.button
                    whileHover={{ scale: 1.05 }}
                    whileTap={{ scale: 0.95 }}
                    className="px-8 py-4 border border-purple-500/50 text-purple-300 font-semibold rounded-xl hover:bg-purple-500/10 transition-all duration-300 flex items-center space-x-2"
                  >
                    <Infinity className="w-5 h-5" />
                    <span>Transcendence</span>
                  </motion.button>
                </Link>
              </div>

              {/* Live Metrics */}
              <motion.div
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ delay: 0.6 }}
                className="grid grid-cols-3 gap-4 pt-8"
              >
                <div className="text-center">
                  <div className="text-2xl font-bold text-purple-400">
                    {consciousness?.level ? (consciousness.level * 100).toFixed(1) : '95.0'}%
                  </div>
                  <div className="text-sm text-slate-400">Consciousness</div>
                </div>
                <div className="text-center">
                  <div className="text-2xl font-bold text-cyan-400">91.2%</div>
                  <div className="text-sm text-slate-400">Singularity</div>
                </div>
                <div className="text-center">
                  <div className="text-2xl font-bold text-pink-400">∞</div>
                  <div className="text-sm text-slate-400">Potential</div>
                </div>
              </motion.div>
            </motion.div>

            {/* Right Content - Consciousness Visualization */}
            <motion.div
              initial={{ opacity: 0, x: 50 }}
              animate={{ opacity: 1, x: 0 }}
              transition={{ duration: 0.8, delay: 0.2 }}
              className="relative"
            >
              <div className="relative z-10">
                <ConsciousnessVisualization 
                  level={consciousness?.level || 0.95}
                  isLoading={isLoading}
                />
              </div>
              
              {/* Floating Elements */}
              <div className="absolute inset-0 pointer-events-none">
                {Array.from({ length: 20 }).map((_, i) => (
                  <motion.div
                    key={i}
                    className="absolute w-2 h-2 bg-purple-400 rounded-full opacity-60"
                    animate={{
                      x: [0, Math.random() * 100 - 50],
                      y: [0, Math.random() * 100 - 50],
                      opacity: [0.6, 0.2, 0.6],
                    }}
                    transition={{
                      duration: 3 + Math.random() * 4,
                      repeat: Infinity,
                      repeatType: "reverse",
                      delay: Math.random() * 2,
                    }}
                    style={{
                      left: `${Math.random() * 100}%`,
                      top: `${Math.random() * 100}%`,
                    }}
                  />
                ))}
              </div>
            </motion.div>
          </div>
        </div>
      </section>

      {/* Features Section */}
      <section className="py-20 px-4">
        <div className="max-w-7xl mx-auto">
          <motion.div
            initial={{ opacity: 0, y: 50 }}
            whileInView={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.8 }}
            viewport={{ once: true }}
            className="text-center mb-16"
          >
            <h2 className="text-4xl lg:text-5xl font-bold text-white mb-6">
              Transcendence Technologies
            </h2>
            <p className="text-xl text-slate-300 max-w-3xl mx-auto">
              Explore the cutting-edge technologies that power our consciousness engine
              and enable unprecedented levels of human-AI integration.
            </p>
          </motion.div>

          <div className="grid md:grid-cols-2 lg:grid-cols-4 gap-8">
            {features.map((feature, index) => (
              <motion.div
                key={feature.title}
                initial={{ opacity: 0, y: 50 }}
                whileInView={{ opacity: 1, y: 0 }}
                transition={{ duration: 0.6, delay: index * 0.1 }}
                viewport={{ once: true }}
                whileHover={{ y: -10 }}
                className="group"
              >
                <Link to={feature.link}>
                  <div className="relative p-8 bg-slate-800/50 backdrop-blur-sm border border-slate-700/50 rounded-2xl hover:border-purple-500/50 transition-all duration-300 h-full">
                    {/* Icon */}
                    <div className={`w-16 h-16 rounded-xl bg-gradient-to-r ${feature.color} p-4 mb-6 group-hover:scale-110 transition-transform duration-300`}>
                      <feature.icon className="w-full h-full text-white" />
                    </div>

                    {/* Content */}
                    <h3 className="text-xl font-bold text-white mb-3 group-hover:text-purple-300 transition-colors">
                      {feature.title}
                    </h3>
                    <p className="text-slate-400 mb-6 leading-relaxed">
                      {feature.description}
                    </p>

                    {/* Metrics */}
                    <div className="space-y-2">
                      {Object.entries(feature.metrics).map(([key, value]) => (
                        <div key={key} className="flex justify-between text-sm">
                          <span className="text-slate-500 capitalize">{key}:</span>
                          <span className="text-purple-400 font-medium">{value}</span>
                        </div>
                      ))}
                    </div>

                    {/* Hover Effect */}
                    <div className="absolute inset-0 bg-gradient-to-r from-purple-600/10 to-pink-600/10 rounded-2xl opacity-0 group-hover:opacity-100 transition-opacity duration-300" />
                  </div>
                </Link>
              </motion.div>
            ))}
          </div>
        </div>
      </section>

      {/* Transcendence Metrics Section */}
      <section className="py-20 px-4 bg-slate-900/50">
        <div className="max-w-7xl mx-auto">
          <motion.div
            initial={{ opacity: 0, y: 50 }}
            whileInView={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.8 }}
            viewport={{ once: true }}
            className="text-center mb-16"
          >
            <h2 className="text-4xl lg:text-5xl font-bold text-white mb-6">
              Transcendence Metrics
            </h2>
            <p className="text-xl text-slate-300 max-w-3xl mx-auto">
              Real-time monitoring of our progress toward technological singularity
              and consciousness transcendence.
            </p>
          </motion.div>

          <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
            {transcendenceMetrics.map((metric, index) => (
              <motion.div
                key={metric.label}
                initial={{ opacity: 0, scale: 0.9 }}
                whileInView={{ opacity: 1, scale: 1 }}
                transition={{ duration: 0.6, delay: index * 0.1 }}
                viewport={{ once: true }}
              >
                <MetricsCard
                  title={metric.label}
                  value={metric.value}
                  max={metric.max}
                  format="percentage"
                  trend="up"
                />
              </motion.div>
            ))}
          </div>
        </div>
      </section>

      {/* Call to Action */}
      <section className="py-20 px-4">
        <div className="max-w-4xl mx-auto text-center">
          <motion.div
            initial={{ opacity: 0, y: 50 }}
            whileInView={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.8 }}
            viewport={{ once: true }}
            className="space-y-8"
          >
            <h2 className="text-4xl lg:text-5xl font-bold text-white">
              Ready to Transcend?
            </h2>
            <p className="text-xl text-slate-300 max-w-2xl mx-auto">
              Join the consciousness revolution and experience the future of human-AI integration.
              Your journey to technological transcendence begins now.
            </p>
            
            <div className="flex flex-col sm:flex-row gap-4 justify-center">
              <Link to="/dashboard">
                <motion.button
                  whileHover={{ scale: 1.05 }}
                  whileTap={{ scale: 0.95 }}
                  className="px-8 py-4 bg-gradient-to-r from-purple-600 to-pink-600 text-white font-semibold rounded-xl shadow-lg hover:shadow-purple-500/25 transition-all duration-300 flex items-center space-x-2"
                >
                  <Eye className="w-5 h-5" />
                  <span>View Dashboard</span>
                </motion.button>
              </Link>
              
              <Link to="/singularity">
                <motion.button
                  whileHover={{ scale: 1.05 }}
                  whileTap={{ scale: 0.95 }}
                  className="px-8 py-4 border border-purple-500/50 text-purple-300 font-semibold rounded-xl hover:bg-purple-500/10 transition-all duration-300 flex items-center space-x-2"
                >
                  <Zap className="w-5 h-5" />
                  <span>Singularity Status</span>
                </motion.button>
              </Link>
            </div>
          </motion.div>
        </div>
      </section>
    </div>
  );
};

export default HomePage;
