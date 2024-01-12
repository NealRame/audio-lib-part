# Audio lib part

Répertorie tous les fichiers audio d'une collection dont le volume moyen est
inférieur à un seuil donné.

Le nom est complement eclaté parce qu'à la base je voulais déplacer les
fichiers en dessous du seuil dans un dossier. Finalement j'ai décidé de juste
les lister.

Bref, il faudrait trouver un meilleur nom, mais flemme!


## Installation

```bash
git clone https://github.com/NealRame/audio-lib-part.git
cd audio-lib-part
cargo install --path SOME_PATH
```

## Usage

### Synopsis
```
audio-lib-part [OPTIONS] <THRESHOLD> <BASE>
```


### Arguments

#### THRESHOLD

Le seuil en LUFS (db). La valeur est automatiquement converti en valeur
négative.

Par exemple une valeur de _14_ est transformée en _-14_.

Plus la valeur est proche de zero plus le volume seuil est fort.

#### BASE
Une chemin vers un fichier ou un dossier.


### Options

#### -i, --include <PATTERN>
Un _Glob Pattern_ pour filtrer les fichiers à analyser. On peut en fournir
plusieurs.

#### -e, --exclude <PATTERN>
Un _Glob Pattern_ pour exclure des fichiers. On peut en fournir plusieurs.

#### --no-ignore-case
Par défaut les _Pattern_ d'inclusion/exclusion ne sont pas sensible à la casse
on peut activer la sensibilité à la casse grâce à cette option.

#### -v, --verbose
Active la verbosité.

#### -h, --help
Affiche l'aide.

#### -V, --version
Affiche la version.
