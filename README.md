# template-monorepo

This is a project to demonstrate tools which can be used to manage a monorepo.

## deps

You'll need to have [please.build](https://please.build/) installed.

---

## platforms

### macOS
I use [colima](https://github.com/abiosoft/colima) to run the app locally.

### linux [WIP]
I use [MicroK8S](https://microk8s.io/) to run the app on Linux.

You'll need to install it and also enable the built-in registry to run the app, which can be done with `microk8s enable registry`

---

### inject secrets
```bash
. ./template.envrc
```

### create services
to deploy the app, create the services

```bash
plz build //svc/...
plz run parallel //svc/api //svc/documentation
```

### [extra step for linux] push images to docker registry
Because the images need to be on the microk8s docker registry for this to work, we need to push the images manually. (need to try and automate this later)
```
plz run parallel //svc/api:api_push //svc/documentation:documentation_push
```

### deploy to kubernetes
```
# create the k8s namespace
kubectl create namespace template-monorepo

plz run parallel //svc/api/k8s:k8s_push //svc/parser/k8s:k8s_push
plz run //svc/documentation/k8s:k8s_push
```

### cleanup
```
plz run parallel //svc/api/k8s:k8s_cleanup //svc/documentation/k8s:k8s_cleanup

kubectl delete namespace template-monorepo
```

### access the documentation site

#### on macos

use this command to forward the port from the documentation service
```bash
kubectl port-forward service/documentation -n $K8S_NAMESPACE 3000:80
```
you can navigate to `http://localhost:3000`

#### on linux
with microk8s on Linux, the services will directly be available on the NodePort so, no need to use port-forward.

You can get the port with 
```
kubectl -n template-monorepo describe svc/documentation-svc | grep NodePort
```
