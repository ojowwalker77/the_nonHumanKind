# Gene Balance Analysis

## Current Ranges:

1. HP Capacity: 50-200 (dominant), 50-125 (recessive)
2. Energy Capacity: 50-275 (dominant), 50-200 (recessive)
3. Movement Speed: 1-6 (dominant), 1-4 (recessive)
4. Natural Energy Loss: 1-8 (dominant), 1-6 (recessive)
5. Active Energy Loss: 5-20 (dominant), 5-12 (recessive)
6. Food Efficiency: 1-6 (dominant), 1-4 (recessive)
7. Reproduction Rate: 0-3 (dominant), 0-2 (recessive)

## Observations:

1. HP and Energy Capacity have much larger ranges compared to other traits.
2. The difference between dominant and recessive expressions is not consistent across traits.
3. Some traits (like Movement Speed and Food Efficiency) have very small ranges, which might not allow for significant variation.
4. Active Energy Loss has a potentially large impact on individual survival.
5. Reproduction Rate has a very small range, which might lead to rapid population growth or decline.

##  Adjustments:

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

## Rationale:

1. Reduced the range of HP and Energy Capacity to be more in line with other traits.
2. Increased the range of Movement Speed and Food Efficiency to allow for more variation.
3. Decreased the impact of Active Energy Loss to prevent it from dominating survival chances.
4. Maintained a similar range for Reproduction Rate but made it slightly harder to achieve the maximum value.
5. Ensured that dominant expressions always have an advantage over recessive ones, but not overwhelmingly so.




#### Implement a system to track the distribution of traits across the population over time.
