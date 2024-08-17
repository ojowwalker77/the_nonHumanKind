# The NonHumankind Simulation: Fine-Tuning Plan v1.0

## 1. Genetic System Overhaul

### 1.1 DNA Structure
- Implement 16-character hexadecimal DNA string
- Each pair of characters represents a gene (8 genes total)
- First character of each pair is dominant, second is recessive

### 1.2 Revised Gene Traits and Ranges
1. HP Capacity:
   - Dominant: value * 5 + 50 (range: 50-125)
   - Recessive: value * 3 + 50 (range: 50-95)

2. Energy Capacity:
   - Dominant: value * 10 + 50 (range: 50-200)
   - Recessive: value * 7 + 50 (range: 50-155)

3. Movement Speed:
   - Dominant: value / 2 + 1 (range: 1-8)
   - Recessive: value / 3 + 1 (range: 1-6)

4. Natural Energy Loss:
   - Dominant: value / 3 + 1 (range: 1-6)
   - Recessive: value / 4 + 1 (range: 1-4)

5. Active Energy Loss:
   - Dominant: value / 2 + 5 (range: 5-12)
   - Recessive: value / 3 + 5 (range: 5-10)

6. Food Efficiency:
   - Dominant: value / 2 + 1 (range: 1-8)
   - Recessive: value / 3 + 1 (range: 1-6)

7. Reproduction Rate:
   - Dominant: value / 4 (range: 0-3)
   - Recessive: value / 6 (range: 0-2)

8. Gender:
   - Dominant (8-F): Male
   - Recessive (0-7): Female

### 1.3 Mutation System
- 10% chance of mutation per gene during reproduction
- 70% chance to mutate to a nearby value (±1-2 in hex)
- 30% chance for a completely random new value

## 2. Environmental Changes

### 2.1 Food System
- Implement maximum carrying capacity for vegetation on each tile
- Grass: Not edible by individuals, regrows every turn up to 10 energy
- Fruit trees: Regrow a fruit every 20 turns, providing 50 energy
- Implement food spread to adjacent tiles

### 2.2 Seasons
- Implement two seasons affecting food growth rates
- Season 1: Normal growth
- Season 2: Reduced growth (50% slower)

## 3. Individual Mechanics

### 3.1 Energy System
- Base natural energy loss: 20 per turn, affected by genes
- Movement energy cost: Determined by active energy loss gene

### 3.2 Reproduction
- Requires two individuals (male and female) on the same tile
- Both must have energy > 75% of capacity (affected by genes)
- Consumes 30% of each parent's energy
- Offspring starts with 50% of the energy consumed by parents

### 3.3 Competition and Fighting
- Implement when two individuals try to eat from the same tile
- Both individuals lose energy
- Loser also loses HP
- Loser forced to move to an adjacent tile

### 3.4 Movement and Pathfinding
- Implement simple pathfinding affected by movement speed gene
- Add "scent" system for detecting food in adjacent tiles (range affected by genes)

## 4. Population Control

### 4.1 Tile Capacity
- Maximum of 2 individuals per tile
- Check capacity before movement, choose another direction if full

## 5. Data Tracking and Analysis

### 5.1 Trait Distribution
- Implement system to track distribution of traits across population over time
- Log data every N turns for later analysis

### 5.2 Population Statistics
- Track total population, births, deaths
- Monitor average energy and HP levels


## 6. Testing Framework

### 6.1 Unit Tests
- Create unit tests for each major component:
  - Genetics
  - Individual
  - Map
  - VegetationSystem
  - IndividualRules
  - MovementRules

### 6.2 Integration Tests
- Develop integration tests to ensure components work together correctly:
  - Reproduction system
  - Competition and fighting mechanics
  - Seasonal changes and their effects

### 6.3 Simulation Scenarios
- Create a set of controlled simulation scenarios to test specific aspects:
  - Overpopulation scenario
  - Resource scarcity scenario
  - Rapid environmental change scenario

### 6.4 Gene Efficiency Validation
- Implement tests to track and analyze the effectiveness of different genetic traits:
  - Survival rate of individuals with specific trait combinations
  - Reproduction success based on genetic makeup
  - Energy efficiency across different genetic profiles

## 7. Environment Configurations

### 7.1 Baseline Environment
- Standard configuration with balanced resources and moderate challenges

### 7.2 Harsh Environment
- Scarce resources
- More frequent seasonal changes
- Higher energy costs for survival

### 7.3 Abundant Environment
- Plentiful resources
- Mild seasonal effects
- Lower energy costs for survival

### 7.4 Dynamic Environment
- Alternating periods of scarcity and abundance
- Unpredictable seasonal changes
- Varying energy costs

### 7.5 Competitive Environment
- Limited resources
- Higher population density
- Increased competition for food and reproduction

## 8. Data Analysis and Visualization

### 8.1 Gene Distribution Tracker
- Implement a system to track and visualize gene distribution over time

### 8.2 Environment-Gene Interaction Analysis
- Create tools to analyze how different genes perform in various environments

### 8.3 Population Dynamics Visualizer
- Develop graphs and charts to show population changes, birth/death rates, and resource consumption

### 8.4 Individual Lifecycle Analyzer
- Implement a system to track and analyze the lifecycle of individuals with different genetic makeups

## 9. Implementation Plan

1. Update `Genetics` struct and methods
2. Modify `Individual` struct to include new genetic system
3. Implement new food system in `Map` and `VegetationSystem`
4. Update `IndividualRules` to include new mechanics (reproduction, competition, fighting)
5. Implement seasonal changes
6. Create data tracking and logging system
7. Develop testing framework and write initial tests
8. Create different environment configurations
9. Implement data analysis and visualization tools
10. Update visualization to reflect new mechanics
11. Conduct extensive testing across different environments
12. Analyze results and adjust genetic ranges and mechanics as needed
