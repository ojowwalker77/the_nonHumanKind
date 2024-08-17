## DNA Structure
- 16-character string
- Each character is a hexadecimal digit (0-9, A-F)
- Each pair of characters represents a gene

## Gene Pairs and Traits

1. HP Capacity (1st pair)
2. Energy Capacity (2nd pair)
3. Movement Speed (3rd pair)
4. Natural Energy Loss (4th pair)
5. Active Energy Loss (5th pair)
6. Food Efficiency (6th pair)
7. Reproduction Rate (7th pair)
8. Reserved for future use (8th pair)

## Dominant and Recessive Genes

- Each gene pair consists of two alleles
- The first character in each pair represents the dominant allele
- The second character represents the recessive allele
- Trait expression is determined by the dominant allele, unless both alleles are recessive

## Trait Calculation

For each trait, we'll use the following system:
- Convert the hexadecimal digit to a decimal number (0-15)
- Apply a scaling factor and offset to get the final trait value
- If both alleles are recessive (0-7), use the recessive value instead

## DNA String Generation

1. For initial population:
   - Generate 16 random hexadecimal digits

2. For offspring (sexual reproduction):
   - For each gene pair:
     - 45% chance to inherit dominant allele from first parent
     - 45% chance to inherit dominant allele from second parent
     - 10% chance of mutation (generate new random hex digit)
   - Repeat the process for recessive alleles

3. Mutation:
   - When a mutation occurs, generate a new random hexadecimal digit
   - 70% chance to mutate to a nearby value (±1-2 in hex)
   - 30% chance for a completely random new value
