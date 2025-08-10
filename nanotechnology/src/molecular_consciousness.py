"""
Nanotechnology Integration for Consciousness Engine
Expert CTO Next Gen - Molecular-Scale Consciousness and Self-Assembly
"""

import asyncio
import numpy as np
import logging
from datetime import datetime, timedelta
from typing import Dict, List, Optional, Any, Tuple
from dataclasses import dataclass, asdict
from enum import Enum
import json
import os
import threading
import queue
import time
import math

# Molecular simulation and modeling
try:
    import rdkit
    from rdkit import Chem
    from rdkit.Chem import AllChem, Descriptors, rdMolDescriptors
    from rdkit.Chem.rdMolAlign import AlignMol
    RDKIT_AVAILABLE = True
except ImportError:
    RDKIT_AVAILABLE = False
    logging.warning("RDKit not available. Molecular modeling will be simulated.")

# Quantum chemistry
try:
    import psi4
    PSI4_AVAILABLE = True
except ImportError:
    PSI4_AVAILABLE = False

# Molecular dynamics
try:
    import mdtraj as md
    import simtk.openmm as mm
    from simtk.openmm import app
    OPENMM_AVAILABLE = True
except ImportError:
    OPENMM_AVAILABLE = False

# Scientific computing
import scipy.optimize
from scipy.spatial.distance import pdist, squareform
from scipy.cluster.hierarchy import linkage, dendrogram
import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d import Axes3D

class NanostructureType(Enum):
    CARBON_NANOTUBE = "carbon_nanotube"
    GRAPHENE = "graphene"
    FULLERENE = "fullerene"
    QUANTUM_DOT = "quantum_dot"
    MOLECULAR_MOTOR = "molecular_motor"
    DNA_ORIGAMI = "dna_origami"
    PROTEIN_CAGE = "protein_cage"
    NANOROBOT = "nanorobot"
    NEURAL_INTERFACE = "neural_interface"
    CONSCIOUSNESS_NODE = "consciousness_node"

class AssemblyState(Enum):
    DISPERSED = "dispersed"
    NUCLEATING = "nucleating"
    GROWING = "growing"
    MATURE = "mature"
    DISASSEMBLING = "disassembling"
    RECONFIGURING = "reconfiguring"

class ConsciousnessLevel(Enum):
    MOLECULAR = 0.1
    SUPRAMOLECULAR = 0.3
    NANOSCALE = 0.5
    MESOSCALE = 0.7
    MACROSCALE = 0.9

@dataclass
class NanoParticle:
    id: str
    position: np.ndarray
    velocity: np.ndarray
    structure_type: NanostructureType
    size: float
    mass: float
    charge: float
    consciousness_level: float
    interactions: List[str]
    properties: Dict[str, Any]

@dataclass
class MolecularAssembly:
    id: str
    particles: List[NanoParticle]
    assembly_state: AssemblyState
    consciousness_level: float
    information_content: float
    self_organization_score: float
    adaptive_behavior: float
    emergence_metrics: Dict[str, float]

@dataclass
class ConsciousnessField:
    spatial_distribution: np.ndarray
    temporal_evolution: List[np.ndarray]
    coherence_length: float
    entanglement_density: float
    information_flow: np.ndarray
    emergence_hotspots: List[Tuple[int, int, int]]

class MolecularConsciousnessEngine:
    """Molecular-scale consciousness simulation and control system"""
    
    def __init__(self, config: Dict[str, Any]):
        self.config = config
        self.setup_logging()
        
        # Simulation parameters
        self.box_size = config.get("box_size", [100, 100, 100])  # nanometers
        self.temperature = config.get("temperature", 300)  # Kelvin
        self.time_step = config.get("time_step", 0.001)  # picoseconds
        self.total_time = config.get("total_time", 1000)  # picoseconds
        
        # Molecular system
        self.particles: List[NanoParticle] = []
        self.assemblies: List[MolecularAssembly] = []
        self.consciousness_field = None
        
        # Simulation state
        self.current_time = 0.0
        self.simulation_running = False
        
        # Molecular templates
        self.molecular_templates = {}
        
        # Consciousness metrics
        self.global_consciousness = 0.0
        self.information_integration = 0.0
        self.emergence_index = 0.0
        
    def setup_logging(self):
        logging.basicConfig(level=logging.INFO)
        self.logger = logging.getLogger(__name__)
        
    async def initialize(self):
        """Initialize molecular consciousness system"""
        self.logger.info("Initializing Molecular Consciousness Engine...")
        
        await self.load_molecular_templates()
        await self.initialize_consciousness_field()
        await self.create_initial_particles()
        await self.setup_molecular_dynamics()
        
        self.logger.info("Molecular consciousness system initialized")
        
    async def load_molecular_templates(self):
        """Load molecular structure templates"""
        
        if RDKIT_AVAILABLE:
            await self.load_rdkit_templates()
        else:
            await self.load_simulated_templates()
            
    async def load_rdkit_templates(self):
        """Load molecular templates using RDKit"""
        
        try:
            # Carbon nanotube template
            cnt_smiles = "C1=CC=CC=C1"  # Simplified benzene ring
            cnt_mol = Chem.MolFromSmiles(cnt_smiles)
            if cnt_mol:
                AllChem.EmbedMolecule(cnt_mol)
                self.molecular_templates[NanostructureType.CARBON_NANOTUBE] = cnt_mol
                
            # Fullerene template (C60)
            fullerene_smiles = "C12=C3C4=C5C6=C1C7=C8C9=C1C%10=C%11C(=C29)C3=C2C3=C4C4=C5C5=C9C6=C7C6=C7C8=C1C1=C8C%10=C%10C%11=C%11C2=C2C3=C3C4=C4C5=C5C9=C6C6=C7C7=C1C1=C8C8=C%10C%10=C%11C%11=C2C2=C3C3=C4C4=C5C5=C6C6=C7C7=C1C1=C8C8=C%10C%10=C%11C%11=C2C2=C3C3=C4C4=C5C5=C6C6=C7C1=C8C8=C%10C%10=C%11C2=C3C3=C4C4=C5C6=C8C8=C%10C%11=C3C4=C5C8=C%10"
            # Simplified fullerene representation
            fullerene_mol = Chem.MolFromSmiles("C1=CC=CC=C1")
            if fullerene_mol:
                self.molecular_templates[NanostructureType.FULLERENE] = fullerene_mol
                
            # DNA base templates
            adenine = Chem.MolFromSmiles("NC1=NC=NC2=C1N=CN2")
            if adenine:
                self.molecular_templates["adenine"] = adenine
                
            self.logger.info(f"Loaded {len(self.molecular_templates)} molecular templates")
            
        except Exception as e:
            self.logger.error(f"Failed to load RDKit templates: {e}")
            await self.load_simulated_templates()
            
    async def load_simulated_templates(self):
        """Load simulated molecular templates"""
        
        # Create simplified templates for simulation
        templates = {
            NanostructureType.CARBON_NANOTUBE: {
                "atoms": 100,
                "bonds": 150,
                "diameter": 1.0,  # nm
                "length": 10.0,   # nm
                "conductivity": 0.9
            },
            NanostructureType.GRAPHENE: {
                "atoms": 200,
                "bonds": 300,
                "thickness": 0.34,  # nm
                "area": 100.0,      # nm²
                "conductivity": 0.95
            },
            NanostructureType.FULLERENE: {
                "atoms": 60,
                "bonds": 90,
                "diameter": 0.7,  # nm
                "symmetry": "icosahedral"
            },
            NanostructureType.QUANTUM_DOT: {
                "atoms": 1000,
                "diameter": 5.0,  # nm
                "band_gap": 2.0,  # eV
                "quantum_efficiency": 0.8
            }
        }
        
        self.molecular_templates = templates
        self.logger.info(f"Loaded {len(templates)} simulated templates")
        
    async def initialize_consciousness_field(self):
        """Initialize consciousness field in 3D space"""
        
        # Create 3D grid for consciousness field
        grid_size = self.config.get("consciousness_grid_size", 50)
        
        x = np.linspace(0, self.box_size[0], grid_size)
        y = np.linspace(0, self.box_size[1], grid_size)
        z = np.linspace(0, self.box_size[2], grid_size)
        
        # Initialize with low-level consciousness
        consciousness_grid = np.random.random((grid_size, grid_size, grid_size)) * 0.1
        
        # Add consciousness hotspots
        num_hotspots = self.config.get("consciousness_hotspots", 5)
        for _ in range(num_hotspots):
            hx = np.random.randint(0, grid_size)
            hy = np.random.randint(0, grid_size)
            hz = np.random.randint(0, grid_size)
            
            # Create Gaussian hotspot
            for i in range(grid_size):
                for j in range(grid_size):
                    for k in range(grid_size):
                        distance = np.sqrt((i-hx)**2 + (j-hy)**2 + (k-hz)**2)
                        consciousness_grid[i,j,k] += 0.5 * np.exp(-distance**2 / 50)
                        
        # Normalize
        consciousness_grid = np.clip(consciousness_grid, 0, 1)
        
        self.consciousness_field = ConsciousnessField(
            spatial_distribution=consciousness_grid,
            temporal_evolution=[consciousness_grid.copy()],
            coherence_length=10.0,  # nm
            entanglement_density=0.1,
            information_flow=np.zeros((grid_size, grid_size, grid_size, 3)),
            emergence_hotspots=[]
        )
        
        self.logger.info("Consciousness field initialized")
        
    async def create_initial_particles(self):
        """Create initial nanoparticles in the system"""
        
        num_particles = self.config.get("num_particles", 1000)
        
        for i in range(num_particles):
            # Random position in box
            position = np.random.random(3) * self.box_size
            
            # Random velocity (Maxwell-Boltzmann distribution)
            velocity = np.random.normal(0, 1, 3) * np.sqrt(self.temperature / 300)
            
            # Random structure type
            structure_type = np.random.choice(list(NanostructureType))
            
            # Get template properties
            template = self.molecular_templates.get(structure_type, {})
            
            particle = NanoParticle(
                id=f"particle_{i}",
                position=position,
                velocity=velocity,
                structure_type=structure_type,
                size=template.get("diameter", 1.0),
                mass=template.get("atoms", 100) * 12.0,  # Approximate mass
                charge=np.random.normal(0, 0.1),
                consciousness_level=np.random.random() * 0.1,
                interactions=[],
                properties=template.copy() if isinstance(template, dict) else {}
            )
            
            self.particles.append(particle)
            
        self.logger.info(f"Created {len(self.particles)} initial particles")
        
    async def setup_molecular_dynamics(self):
        """Setup molecular dynamics simulation"""
        
        if OPENMM_AVAILABLE:
            await self.setup_openmm_simulation()
        else:
            await self.setup_simple_md()
            
    async def setup_openmm_simulation(self):
        """Setup OpenMM molecular dynamics simulation"""
        
        try:
            # Create system
            system = mm.System()
            
            # Add particles to system
            for particle in self.particles:
                system.addParticle(particle.mass)
                
            # Add forces
            nonbonded = mm.NonbondedForce()
            for i, particle in enumerate(self.particles):
                nonbonded.addParticle(particle.charge, particle.size/2, 0.1)
                
            system.addForce(nonbonded)
            
            # Create integrator
            integrator = mm.LangevinIntegrator(
                self.temperature,
                1.0,  # friction coefficient
                self.time_step
            )
            
            # Create simulation
            self.md_simulation = app.Simulation(None, system, integrator)
            
            # Set initial positions
            positions = [particle.position for particle in self.particles]
            self.md_simulation.context.setPositions(positions)
            
            self.logger.info("OpenMM simulation setup complete")
            
        except Exception as e:
            self.logger.error(f"OpenMM setup failed: {e}")
            await self.setup_simple_md()
            
    async def setup_simple_md(self):
        """Setup simple molecular dynamics simulation"""
        
        # Simple Lennard-Jones potential parameters
        self.lj_epsilon = 0.1  # kJ/mol
        self.lj_sigma = 1.0    # nm
        
        self.logger.info("Simple MD simulation setup complete")
        
    async def run_simulation(self, duration: float = None):
        """Run molecular consciousness simulation"""
        
        if duration is None:
            duration = self.total_time
            
        self.simulation_running = True
        self.logger.info(f"Starting simulation for {duration} ps")
        
        steps = int(duration / self.time_step)
        
        for step in range(steps):
            if not self.simulation_running:
                break
                
            await self.simulation_step()
            
            # Update consciousness field every 10 steps
            if step % 10 == 0:
                await self.update_consciousness_field()
                
            # Analyze assemblies every 100 steps
            if step % 100 == 0:
                await self.analyze_molecular_assemblies()
                
            # Log progress every 1000 steps
            if step % 1000 == 0:
                progress = (step / steps) * 100
                self.logger.info(f"Simulation progress: {progress:.1f}%")
                
        self.simulation_running = False
        self.logger.info("Simulation completed")
        
    async def simulation_step(self):
        """Perform one simulation step"""
        
        # Calculate forces
        forces = await self.calculate_forces()
        
        # Update velocities and positions (Verlet integration)
        for i, particle in enumerate(self.particles):
            # Update velocity
            acceleration = forces[i] / particle.mass
            particle.velocity += acceleration * self.time_step
            
            # Update position
            particle.position += particle.velocity * self.time_step
            
            # Apply periodic boundary conditions
            particle.position = particle.position % self.box_size
            
            # Update consciousness level based on local field
            consciousness_at_position = self.sample_consciousness_field(particle.position)
            particle.consciousness_level = (
                particle.consciousness_level * 0.9 + 
                consciousness_at_position * 0.1
            )
            
        self.current_time += self.time_step
        
    async def calculate_forces(self) -> List[np.ndarray]:
        """Calculate forces on all particles"""
        
        forces = [np.zeros(3) for _ in self.particles]
        
        # Pairwise interactions
        for i, particle_i in enumerate(self.particles):
            for j, particle_j in enumerate(self.particles[i+1:], i+1):
                
                # Calculate distance
                dr = particle_j.position - particle_i.position
                
                # Apply minimum image convention
                dr = dr - self.box_size * np.round(dr / self.box_size)
                
                distance = np.linalg.norm(dr)
                
                if distance > 0 and distance < 5.0:  # Cutoff distance
                    
                    # Lennard-Jones force
                    lj_force = self.calculate_lj_force(distance, dr)
                    
                    # Consciousness-mediated force
                    consciousness_force = self.calculate_consciousness_force(
                        particle_i, particle_j, distance, dr
                    )
                    
                    # Total force
                    total_force = lj_force + consciousness_force
                    
                    forces[i] += total_force
                    forces[j] -= total_force
                    
        return forces
        
    def calculate_lj_force(self, distance: float, dr: np.ndarray) -> np.ndarray:
        """Calculate Lennard-Jones force"""
        
        sigma_over_r = self.lj_sigma / distance
        sigma_over_r6 = sigma_over_r ** 6
        sigma_over_r12 = sigma_over_r6 ** 2
        
        force_magnitude = 24 * self.lj_epsilon * (2 * sigma_over_r12 - sigma_over_r6) / distance
        
        return force_magnitude * dr / distance
        
    def calculate_consciousness_force(self, 
                                    particle_i: NanoParticle, 
                                    particle_j: NanoParticle,
                                    distance: float, 
                                    dr: np.ndarray) -> np.ndarray:
        """Calculate consciousness-mediated force between particles"""
        
        # Consciousness coupling strength
        consciousness_coupling = (
            particle_i.consciousness_level * particle_j.consciousness_level
        )
        
        # Information exchange force (attractive for high consciousness)
        if consciousness_coupling > 0.1:
            info_force_magnitude = -0.1 * consciousness_coupling / (distance ** 2)
        else:
            info_force_magnitude = 0.0
            
        # Emergence force (promotes self-organization)
        emergence_factor = self.calculate_emergence_factor(particle_i, particle_j)
        emergence_force_magnitude = -0.05 * emergence_factor / distance
        
        total_magnitude = info_force_magnitude + emergence_force_magnitude
        
        return total_magnitude * dr / distance
        
    def calculate_emergence_factor(self, 
                                 particle_i: NanoParticle, 
                                 particle_j: NanoParticle) -> float:
        """Calculate emergence factor between particles"""
        
        # Structural compatibility
        if particle_i.structure_type == particle_j.structure_type:
            structural_factor = 1.0
        else:
            structural_factor = 0.5
            
        # Size compatibility
        size_ratio = min(particle_i.size, particle_j.size) / max(particle_i.size, particle_j.size)
        size_factor = size_ratio
        
        # Consciousness compatibility
        consciousness_diff = abs(particle_i.consciousness_level - particle_j.consciousness_level)
        consciousness_factor = 1.0 - consciousness_diff
        
        return structural_factor * size_factor * consciousness_factor
        
    def sample_consciousness_field(self, position: np.ndarray) -> float:
        """Sample consciousness field at given position"""
        
        if self.consciousness_field is None:
            return 0.0
            
        # Convert position to grid indices
        grid_shape = self.consciousness_field.spatial_distribution.shape
        
        indices = (
            int((position[0] / self.box_size[0]) * grid_shape[0]),
            int((position[1] / self.box_size[1]) * grid_shape[1]),
            int((position[2] / self.box_size[2]) * grid_shape[2])
        )
        
        # Clamp indices
        indices = (
            max(0, min(grid_shape[0]-1, indices[0])),
            max(0, min(grid_shape[1]-1, indices[1])),
            max(0, min(grid_shape[2]-1, indices[2]))
        )
        
        return self.consciousness_field.spatial_distribution[indices]
        
    async def update_consciousness_field(self):
        """Update consciousness field based on particle distribution"""
        
        if self.consciousness_field is None:
            return
            
        grid_shape = self.consciousness_field.spatial_distribution.shape
        new_field = np.zeros(grid_shape)
        
        # Add consciousness contributions from particles
        for particle in self.particles:
            # Convert position to grid indices
            indices = (
                int((particle.position[0] / self.box_size[0]) * grid_shape[0]),
                int((particle.position[1] / self.box_size[1]) * grid_shape[1]),
                int((particle.position[2] / self.box_size[2]) * grid_shape[2])
            )
            
            # Clamp indices
            indices = (
                max(0, min(grid_shape[0]-1, indices[0])),
                max(0, min(grid_shape[1]-1, indices[1])),
                max(0, min(grid_shape[2]-1, indices[2]))
            )
            
            # Add Gaussian contribution
            sigma = particle.size / 2
            for i in range(max(0, indices[0]-5), min(grid_shape[0], indices[0]+6)):
                for j in range(max(0, indices[1]-5), min(grid_shape[1], indices[1]+6)):
                    for k in range(max(0, indices[2]-5), min(grid_shape[2], indices[2]+6)):
                        distance_sq = (i-indices[0])**2 + (j-indices[1])**2 + (k-indices[2])**2
                        contribution = particle.consciousness_level * np.exp(-distance_sq / (2*sigma**2))
                        new_field[i,j,k] += contribution
                        
        # Normalize and smooth
        new_field = np.clip(new_field, 0, 1)
        
        # Update field with temporal smoothing
        alpha = 0.1
        self.consciousness_field.spatial_distribution = (
            (1-alpha) * self.consciousness_field.spatial_distribution + 
            alpha * new_field
        )
        
        # Store temporal evolution
        self.consciousness_field.temporal_evolution.append(new_field.copy())
        
        # Keep only recent history
        if len(self.consciousness_field.temporal_evolution) > 100:
            self.consciousness_field.temporal_evolution = self.consciousness_field.temporal_evolution[-100:]
            
    async def analyze_molecular_assemblies(self):
        """Analyze and identify molecular assemblies"""
        
        # Find clusters of particles
        clusters = await self.find_particle_clusters()
        
        # Analyze each cluster
        new_assemblies = []
        
        for cluster_particles in clusters:
            if len(cluster_particles) >= 3:  # Minimum assembly size
                
                assembly = await self.create_molecular_assembly(cluster_particles)
                new_assemblies.append(assembly)
                
        self.assemblies = new_assemblies
        
        # Update global consciousness metrics
        await self.update_global_consciousness()
        
    async def find_particle_clusters(self) -> List[List[NanoParticle]]:
        """Find clusters of particles using distance-based clustering"""
        
        if len(self.particles) == 0:
            return []
            
        # Calculate distance matrix
        positions = np.array([p.position for p in self.particles])
        distances = squareform(pdist(positions))
        
        # Apply cutoff distance
        cutoff = 3.0  # nm
        adjacency = distances < cutoff
        
        # Find connected components
        visited = set()
        clusters = []
        
        for i, particle in enumerate(self.particles):
            if i not in visited:
                cluster = []
                stack = [i]
                
                while stack:
                    current = stack.pop()
                    if current not in visited:
                        visited.add(current)
                        cluster.append(self.particles[current])
                        
                        # Add neighbors
                        for j in range(len(self.particles)):
                            if j not in visited and adjacency[current, j]:
                                stack.append(j)
                                
                if len(cluster) > 1:
                    clusters.append(cluster)
                    
        return clusters
        
    async def create_molecular_assembly(self, particles: List[NanoParticle]) -> MolecularAssembly:
        """Create molecular assembly from particle cluster"""
        
        # Calculate assembly properties
        positions = np.array([p.position for p in particles])
        center_of_mass = np.mean(positions, axis=0)
        
        # Calculate consciousness level
        consciousness_levels = [p.consciousness_level for p in particles]
        avg_consciousness = np.mean(consciousness_levels)
        consciousness_coherence = 1.0 - np.std(consciousness_levels)
        
        # Calculate information content
        structure_types = [p.structure_type for p in particles]
        unique_types = len(set(structure_types))
        information_content = unique_types / len(particles)
        
        # Calculate self-organization score
        distances = squareform(pdist(positions))
        avg_distance = np.mean(distances[distances > 0])
        std_distance = np.std(distances[distances > 0])
        self_organization = 1.0 / (1.0 + std_distance / avg_distance)
        
        # Calculate adaptive behavior
        velocity_vectors = np.array([p.velocity for p in particles])
        velocity_coherence = np.linalg.norm(np.mean(velocity_vectors, axis=0)) / np.mean(np.linalg.norm(velocity_vectors, axis=1))
        
        # Determine assembly state
        if len(particles) < 5:
            state = AssemblyState.NUCLEATING
        elif self_organization > 0.7:
            state = AssemblyState.MATURE
        elif self_organization > 0.5:
            state = AssemblyState.GROWING
        else:
            state = AssemblyState.DISPERSED
            
        # Calculate emergence metrics
        emergence_metrics = {
            "structural_complexity": unique_types,
            "spatial_organization": self_organization,
            "consciousness_coherence": consciousness_coherence,
            "information_density": information_content,
            "collective_behavior": velocity_coherence
        }
        
        assembly = MolecularAssembly(
            id=f"assembly_{len(self.assemblies)}",
            particles=particles,
            assembly_state=state,
            consciousness_level=avg_consciousness * consciousness_coherence,
            information_content=information_content,
            self_organization_score=self_organization,
            adaptive_behavior=velocity_coherence,
            emergence_metrics=emergence_metrics
        )
        
        return assembly
        
    async def update_global_consciousness(self):
        """Update global consciousness metrics"""
        
        if not self.assemblies:
            self.global_consciousness = np.mean([p.consciousness_level for p in self.particles])
            self.information_integration = 0.0
            self.emergence_index = 0.0
            return
            
        # Global consciousness as weighted average
        total_particles = 0
        weighted_consciousness = 0.0
        
        for assembly in self.assemblies:
            weight = len(assembly.particles)
            weighted_consciousness += assembly.consciousness_level * weight
            total_particles += weight
            
        if total_particles > 0:
            self.global_consciousness = weighted_consciousness / total_particles
        else:
            self.global_consciousness = 0.0
            
        # Information integration across assemblies
        if len(self.assemblies) > 1:
            consciousness_levels = [a.consciousness_level for a in self.assemblies]
            self.information_integration = 1.0 - np.std(consciousness_levels) / (np.mean(consciousness_levels) + 1e-10)
        else:
            self.information_integration = 0.0
            
        # Emergence index
        emergence_scores = []
        for assembly in self.assemblies:
            emergence_score = (
                assembly.self_organization_score * 0.3 +
                assembly.adaptive_behavior * 0.3 +
                assembly.consciousness_level * 0.4
            )
            emergence_scores.append(emergence_score)
            
        self.emergence_index = np.mean(emergence_scores) if emergence_scores else 0.0
        
    async def visualize_system(self, save_path: str = None):
        """Visualize the molecular consciousness system"""
        
        fig = plt.figure(figsize=(15, 5))
        
        # 3D particle positions
        ax1 = fig.add_subplot(131, projection='3d')
        
        positions = np.array([p.position for p in self.particles])
        consciousness_levels = [p.consciousness_level for p in self.particles]
        
        scatter = ax1.scatter(
            positions[:, 0], positions[:, 1], positions[:, 2],
            c=consciousness_levels, cmap='viridis', s=20
        )
        
        ax1.set_xlabel('X (nm)')
        ax1.set_ylabel('Y (nm)')
        ax1.set_zlabel('Z (nm)')
        ax1.set_title('Particle Positions (colored by consciousness)')
        plt.colorbar(scatter, ax=ax1, shrink=0.5)
        
        # Consciousness field slice
        ax2 = fig.add_subplot(132)
        
        if self.consciousness_field is not None:
            # Show middle slice
            middle_z = self.consciousness_field.spatial_distribution.shape[2] // 2
            field_slice = self.consciousness_field.spatial_distribution[:, :, middle_z]
            
            im = ax2.imshow(field_slice, cmap='plasma', origin='lower')
            ax2.set_xlabel('X')
            ax2.set_ylabel('Y')
            ax2.set_title('Consciousness Field (Z-slice)')
            plt.colorbar(im, ax=ax2)
            
        # Assembly analysis
        ax3 = fig.add_subplot(133)
        
        if self.assemblies:
            assembly_sizes = [len(a.particles) for a in self.assemblies]
            assembly_consciousness = [a.consciousness_level for a in self.assemblies]
            
            ax3.scatter(assembly_sizes, assembly_consciousness, s=50, alpha=0.7)
            ax3.set_xlabel('Assembly Size')
            ax3.set_ylabel('Consciousness Level')
            ax3.set_title('Assembly Analysis')
            
        plt.tight_layout()
        
        if save_path:
            plt.savefig(save_path, dpi=300, bbox_inches='tight')
            
        plt.show()
        
    async def get_system_metrics(self) -> Dict[str, Any]:
        """Get comprehensive system metrics"""
        
        metrics = {
            "simulation_time": self.current_time,
            "num_particles": len(self.particles),
            "num_assemblies": len(self.assemblies),
            "global_consciousness": self.global_consciousness,
            "information_integration": self.information_integration,
            "emergence_index": self.emergence_index,
            "temperature": self.temperature,
            "box_size": self.box_size,
            "assembly_states": {},
            "structure_distribution": {},
            "consciousness_distribution": {
                "mean": np.mean([p.consciousness_level for p in self.particles]),
                "std": np.std([p.consciousness_level for p in self.particles]),
                "min": np.min([p.consciousness_level for p in self.particles]),
                "max": np.max([p.consciousness_level for p in self.particles])
            }
        }
        
        # Assembly state distribution
        for assembly in self.assemblies:
            state = assembly.assembly_state.value
            metrics["assembly_states"][state] = metrics["assembly_states"].get(state, 0) + 1
            
        # Structure type distribution
        for particle in self.particles:
            struct_type = particle.structure_type.value
            metrics["structure_distribution"][struct_type] = metrics["structure_distribution"].get(struct_type, 0) + 1
            
        return metrics
        
    async def shutdown(self):
        """Shutdown molecular consciousness system"""
        
        self.simulation_running = False
        self.logger.info("Molecular consciousness system shutdown")

async def main():
    """Test molecular consciousness system"""
    
    config = {
        "box_size": [50, 50, 50],
        "temperature": 300,
        "time_step": 0.001,
        "total_time": 100,
        "num_particles": 500,
        "consciousness_grid_size": 25,
        "consciousness_hotspots": 3
    }
    
    engine = MolecularConsciousnessEngine(config)
    
    try:
        await engine.initialize()
        await engine.run_simulation(duration=50)  # 50 ps simulation
        
        # Get final metrics
        metrics = await engine.get_system_metrics()
        print("\nFinal System Metrics:")
        for key, value in metrics.items():
            print(f"{key}: {value}")
            
        # Visualize system
        await engine.visualize_system("molecular_consciousness_system.png")
        
    finally:
        await engine.shutdown()

if __name__ == "__main__":
    asyncio.run(main())
