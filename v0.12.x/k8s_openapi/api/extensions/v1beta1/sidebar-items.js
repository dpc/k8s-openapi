initSidebarItems({"enum":[["ReadNamespacedIngressResponse","Use `<ReadNamespacedIngressResponse as Response>::try_from_parts` to parse the HTTP response body of [`Ingress::read_namespaced_ingress`]"],["ReadNamespacedIngressStatusResponse","Use `<ReadNamespacedIngressStatusResponse as Response>::try_from_parts` to parse the HTTP response body of [`Ingress::read_namespaced_ingress_status`]"]],"struct":[["HTTPIngressPath","HTTPIngressPath associates a path with a backend. Incoming urls matching the path are forwarded to the backend."],["HTTPIngressRuleValue","HTTPIngressRuleValue is a list of http selectors pointing to backends. In the example: http://<host>/<path>?<searchpart> -> backend where where parts of the url correspond to RFC 3986, this resource will be used to match against everything after the last ‘/’ and before the first ‘?’ or ‘#’."],["Ingress","Ingress is a collection of rules that allow inbound connections to reach the endpoints defined by a backend. An Ingress can be configured to give services externally-reachable urls, load balance traffic, terminate SSL, offer name based virtual hosting etc. DEPRECATED - This group version of Ingress is deprecated by networking.k8s.io/v1beta1 Ingress. See the release notes for more information."],["IngressBackend","IngressBackend describes all endpoints for a given service and port."],["IngressRule","IngressRule represents the rules mapping the paths under a specified host to the related backend services. Incoming requests are first evaluated for a host match, then routed to the backend associated with the matching IngressRuleValue."],["IngressSpec","IngressSpec describes the Ingress the user wishes to exist."],["IngressStatus","IngressStatus describe the current state of the Ingress."],["IngressTLS","IngressTLS describes the transport layer security associated with an Ingress."],["ReadNamespacedIngressOptional","Optional parameters of [`Ingress::read_namespaced_ingress`]"],["ReadNamespacedIngressStatusOptional","Optional parameters of [`Ingress::read_namespaced_ingress_status`]"]]});