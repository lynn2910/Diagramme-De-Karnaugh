# Diagramme de Karnaugh

## C'est quoi

Le diagramme de karnaugh est une méthode graphique pour trouver ou simplifier une fonction logique à partir de sa table de vérité.

> [Wikipédia](https://fr.wikipedia.org/wiki/Table_de_Karnaugh)

## Pourquoi l'implémenter

Cette méthode est graphique et plutôt "humaine", cependant, je me suis donné pour défi d'implémenter cette méthode afin de pouvoir mieux cerner les rouages de cette méthode et m'entrainer.

## Son efficacité

L'algorithme semble rapide et efficace, cependant je dois admettre que de nombreuses optimisations sont requises avant de pouvoir déclarer cet algorithme comme "optimal".


## Tester le programme

*Pour le moment, il n'est pas possible de donner un diagramme de Karnaugh au début, le diagramme est pré-défini dans le code.*

Installer le code source:
```bash
git clone https://github.com/lynn2910/Diagramme-De-Karnaugh.git
cd Diagramme-De-Karnaugh
```

Veillez à l'existence de la commande `cargo` et de ses dépendances. (compilateur Rust et gestionnaire de librairie)
```bash
cargo --version
```
Versions des dépendances utilisées pendant le développement et test de l'algorithme:
```bash
cargo --version
# cargo 1.74.1 (ecb9851af 2023-10-18)
rustup --version
# rustup 1.26.0 (5af9b9484 2023-04-05)
rustc --version
# rustc 1.74.1 (a28077b28 2023-12-04)
```

Compiler et executer le programme :
```bash
cargo run --release
```

### Fonctionnalités activables :
- `time_calc`: Affiche les temps d'execution et de traitement des données

- `draw_table` : Affiche le diagramme de Karnaugh au départ

- `verbose`: Le programme a envie de discuter (plus de messages)

Activer une ou des fonctionnalités à la compilation :
```bash
cargo build --release --features "feature1 feature2"
```

Exemple de sortie avec toutes les fonctionnalités activées:
```
Diagramme de Karnaugh:
┌───────┬────┬────┬────┬────┐
│ cd/ab │ 00 │ 01 │ 11 │ 10 │
├───────┼────┼────┼────┼────┤
│ 00    │ 0  │ 1  │ 1  │ 0  │
├───────┼────┼────┼────┼────┤
│ 01    │ 1  │ 0  │ 0  │ 1  │
├───────┼────┼────┼────┼────┤
│ 11    │ 1  │ 0  │ 0  │ 1  │
├───────┼────┼────┼────┼────┤
│ 10    │ 0  │ 1  │ 1  │ 0  │
└───────┴────┴────┴────┴────┘
Calcule en cours des blocs...
Calculs des blocs (of 0 & 1) effectué en 0.000001353s
Forme minimale disjonctive: !bd + b!d
Forme minimale conjonctive: (b!d)(!bd)
Les formes disjonctives et conjonctives ont été déterminées en 0.000008767s
```

*Test effectué sur un ordinateur avec ces caractéristiques:*
```
CPU: Ryzen 7 7840HS (8 core, 16 threads)
RAM: 16Gb DDR5 5600 Mhz
NVME: 2To 7.0Gb/s Read 5.0 Gb/s Write
```
