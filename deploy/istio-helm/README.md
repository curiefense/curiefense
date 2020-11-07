# Curiefense-enabled istio mesh deployment with helm

This folder contains helm charts that deploy a curiefense-enabled istio mesh.

Based on the [Istio 1.5.10 release](https://github.com/istio/istio/releases/tag/1.5.10):

* [crds/](crds/) contains Istio 1.5.10 Custom Resource Definitions (folder `install/kubernetes/helm/istio-init/files` in the Istio 1.5.10 release)
* [chart/](chart/) contains the Istio 1.5.10 charts, slightly modified to add curiefense components (folder `install/kubernetes/helm/istio` in the Istio 1.5.10 release)



