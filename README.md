# Stoichiometry calculator

Command-line program to perform simple stoichiometric computations

#### Example execution

```

 -------------------- Stoichiometry calculator CLI --------------------

balance <equation> - balance the equation, e.g. 'balance H2 + O2 => H2O'
compute <equation> - compute the amounts of products, e.g. 'compute 1 mol H2 + 0.5 g O2 => H2O'
mass <molecule> - display the atomic mass of the molecule in uma
exit - exit the program
help - display the current explanations

> compute 1.7 mol C6H12O6 + 100 g O2 => H2O + CO2
1.700 mol C6H12O6 + 3.125 mol O2 => 3.125 mol H2O + 3.125 mol CO2
306.224 g C6H12O6 + 100.000 g O2 => 56.288 g H2O + 137.537 g CO2
limiting reactant: O2

> balance Al2(CO3)3 + H3PO4 => AlPO4 + CO2 + H2O
Al2(CO3)3 + 2 H3PO4 => 2 AlPO4 + 3 CO2 + 3 H2O

>
```

#### Molecule format

`(<atom>[<coef>])* [^<charge>]`

$CO_2$ : `CO2`

$Cu^{2+}$ : `Cu^2+`

$Al_2 (CO_3)_3$ : `Al2(CO3)3`

#### Periodic table data source
GoodmanSciences, Github, https://gist.github.com/GoodmanSciences/c2dd862cd38f21b0ad36b8f96b4bf1ee
