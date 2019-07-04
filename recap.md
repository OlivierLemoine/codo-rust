## Requirement pour le coding dojo sur Linux:

-   BDD : `Mysql`
-   Lib pour pouvoir utiliser Mysql : `libmysqlclient-dev`
-   Pour Rust : Cargo

Tout est installé automatiquement avec avec le script `install.sh` sauf cargo. Pour installer cargo, il faut executer la commande `curl https://sh.rustup.rs -sSf | sh` et choisir l'option `1`.

Le script d'installation à préparé la base de donnée :

-   Une database `TodoDb` est ajoutée
-   Un nouvel utilisateur nommé `diesel` (l'ORM) est crée avec les privilèges sur `TodoDb`
-   La table `todos` est finnalement créée

Le code est disponible dans le fichier `base.sql`. On peut toujours executer la commande `sudo sql < base.sql` pour remettre à zero la BDD.

## Rust

Une fois Cargo installé, ouvrir le dossier du coding dojo.

Deux commandes cargo utiles :

-   `cargo run` compile et execute le code
-   `cargo build` compile le code
-   `cargo check` indique les erreurs sans compiler le code

Dans tous les cas, cargo télécharge et compile les dépendances si il ne les à pas déjà récupérées. Cargo indique aussi toutes les erreurs qui empêcheraient le programme de compiler.

# Rust

C'est un langage :

-   Compilé = performance au niveau de c,c++ (avec des features d'un langage de plus haut niveau)
-   Fortement typé
-   Impératif
-   Multi-paradigme mais non orienté objet. Il emprunte beaucoup aux languages fonctionnelles.
-   Memory safe

## Comment memory safe et compilé ?

Contrairement à Go, pas de garbage collector. À la place, des règles stricts qui empêche la compilation de code potentiellement dangeureux :

-   Ownership : Il ne peut y avoir qu'un seul propriétaire d'une donnée à la fois.
-   Mutability :
-   Lifespan :

## Macro
