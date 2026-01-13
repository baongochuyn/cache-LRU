# Cache LRU Rust

Ce projet est une bibliothèque Rust qui fournit une implémentation générique d'un cache LRU (Least Recently Used) avec persistance sur disque.

## Qu'est-ce qu'un cache LRU ?
Un cache LRU est une structure de données qui stocke les éléments les plus récemment utilisés jusqu'à une capacité maximale. Lorsqu'un nouvel élément est ajouté et que le cache est plein, l'élément le moins récemment utilisé est supprimé.

### Exemple d'utilisation
Supposons un cache de taille 3 :
- On insère A, B, C → le cache contient [A, B, C]
- On insère D → le cache contient [B, C, D] (A est supprimé)
- On lit B → B devient le plus récent [C, D, B] 
- On insère E → le cache contient [D, B, E]

## Fonctionnalités
- Clé et valeur génériques (K, V)
- Trait `CacheLRU` pour abstraction
- Politique d'éviction LRU efficace
- Persistance sur disque (fichier texte)
- Benchmarks de performance (Criterion)
- Tests unitaires et d'intégration

## Utilisation
```rust
use cache::cache::*;
let mut cache = Cache::<String, String>::new(3);
cache.put("A".to_string(), "value_a".to_string());
let val = cache.get(&"A".to_string());
```

### Persistance
```rust
let mut cache = Cache::<String, String>::new_persistent(3, "mon_cache.txt");
cache.put("A".to_string(), "value_a".to_string());
cache.persist().unwrap();
```

## Tests
Lancez tous les tests avec :
```
cargo test
```

## Benchmarks
Lancez les benchmarks avec :
```
cargo bench
```

## Résultats des benchmarks

- `cache get` (LRU): environ 661–686 µs
- `hashmap get` (HashMap pur): environ 298–301 µs

**Explication** :
- LRU cache est plus lent que HashMap car il doit mettre à jour l'ordre d'accès à chaque lecture (pour la politique d'éviction).
- HashMap pur ne fait qu'une recherche, donc plus rapide.
- La différence reste acceptable pour la plupart des usages où la gestion LRU est nécessaire.

## Suggestions pour améliorer la performance ou étendre la bibliothèque

- **Support d'autres politiques d'éviction** :
  - **FIFO (First In First Out)** : Éjecter l'élément le plus ancien, sans tenir compte de l'accès récent.
  - Pour cela, vous pouvez :
    - Définir un trait `CachePolicy` et des structs implémentant LRU, LFU, FIFO...
    - Permettre à l'utilisateur de choisir la politique lors de la création du cache.
    - Benchmarker chaque politique pour comparer.

- **Persistance avancée** :
  - Supporter la sérialisation binaire ou JSON pour des types de données plus complexes.
  - Ajouter la possibilité de charger/sauvegarder en arrière-plan (async).

## Documentation
- Le module principal est `cache`.
- Les méthodes principales sont :
  - `put(key, value)` : insère ou met à jour une entrée
  - `get(key)` : récupère une valeur et met à jour l'ordre LRU
  - `persist()` : sauvegarde le cache sur disque
  - `load_persistent()` : charge le cache depuis un fichier
