# omnidux

A framework for building micro frontend architecture in a fully reactive approach. 


## Preamble

## Terminology

### Scope
A scope is describing a context in which a certain object is available.

### Namespace
A namespace is an optional scope around multiple hulls to enforce independence. It is recommended to at least have two scopes to split user private data and public data (`safety-scope` and `public-scope`). Can be also nested.

### Hull
A container that holds a subset of the application. What belongs is in a hull depends on the application is up for interpretation but usually you would scope it on a feature or entity level.

### Story

### Task
A container simillar to an hull but without any visible UI. A task must be scoped by scope

### Capsule
A data object that holds information required by the UI to display or to control further flow. Capsules have two important properties besides its actual content.

#### Lifetime
The lifetime property of a capsule describes when and how long it is available. Usually used by sensitive data that should be automatically cleared once a certain task has been fulfilled. A lifetime can be defined by time or by action fulfillment.

#### Availability
The availability property of a capsule describes where it is available, it can be defined by scope.


## Examples

### Good old todo app
Of course we do have an todo app example! Yes its totally overengineered as these small projects are not even close to any real life frontend applications. Its only purpose is to show a working app written with omnidux.

## API

