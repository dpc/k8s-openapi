// Generated from definition io.k8s.api.core.v1.Pod

/// Pod is a collection of containers that can run on a host. This resource is created by clients and scheduled onto hosts.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Pod {
    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    pub metadata: crate::apimachinery::pkg::apis::meta::v1::ObjectMeta,

    /// Specification of the desired behavior of the pod. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    pub spec: Option<crate::api::core::v1::PodSpec>,

    /// Most recently observed status of the pod. This data may not be up to date. Populated by the system. Read-only. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    pub status: Option<crate::api::core::v1::PodStatus>,
}

// Begin /v1/Pod

// Generated from operation connectCoreV1DeleteNamespacedPodProxy

impl Pod {
    /// connect DELETE requests to proxy of Pod
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the PodProxyOptions
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn connect_delete_proxy(
        name: &str,
        namespace: &str,
        optional: ConnectDeletePodProxyOptional<'_>,
    ) -> Result<crate::http::Request<Vec<u8>>, crate::RequestError> {
        let ConnectDeletePodProxyOptional {
            path,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/proxy?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        if let Some(path) = path {
            __query_pairs.append_pair("path", path);
        }
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::delete(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`Pod::connect_delete_proxy`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectDeletePodProxyOptional<'a> {
    /// Path is the URL path to use for the current proxy request to pod.
    pub path: Option<&'a str>,
}

// Generated from operation connectCoreV1DeleteNamespacedPodProxyWithPath

impl Pod {
    /// connect DELETE requests to proxy of Pod
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the PodProxyOptions
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `path`
    ///
    ///     path to the resource
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn connect_delete_proxy_with_path(
        name: &str,
        namespace: &str,
        path: &str,
        optional: ConnectDeletePodProxyWithPathOptional<'_>,
    ) -> Result<crate::http::Request<Vec<u8>>, crate::RequestError> {
        let ConnectDeletePodProxyWithPathOptional {
            path_,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/proxy/{path}?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            path = crate::percent_encoding::percent_encode(path.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        if let Some(path_) = path_ {
            __query_pairs.append_pair("path", path_);
        }
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::delete(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`Pod::connect_delete_proxy_with_path`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectDeletePodProxyWithPathOptional<'a> {
    /// Path is the URL path to use for the current proxy request to pod.
    pub path_: Option<&'a str>,
}

// Generated from operation connectCoreV1GetNamespacedPodAttach

impl Pod {
    /// connect GET requests to attach of Pod
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the PodAttachOptions
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn connect_get_attach(
        name: &str,
        namespace: &str,
        optional: ConnectGetPodAttachOptional<'_>,
    ) -> Result<crate::http::Request<Vec<u8>>, crate::RequestError> {
        let ConnectGetPodAttachOptional {
            container,
            stderr,
            stdin,
            stdout,
            tty,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/attach?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        if let Some(container) = container {
            __query_pairs.append_pair("container", container);
        }
        if let Some(stderr) = stderr {
            __query_pairs.append_pair("stderr", if stderr { "true" } else { "false" });
        }
        if let Some(stdin) = stdin {
            __query_pairs.append_pair("stdin", if stdin { "true" } else { "false" });
        }
        if let Some(stdout) = stdout {
            __query_pairs.append_pair("stdout", if stdout { "true" } else { "false" });
        }
        if let Some(tty) = tty {
            __query_pairs.append_pair("tty", if tty { "true" } else { "false" });
        }
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`Pod::connect_get_attach`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectGetPodAttachOptional<'a> {
    /// The container in which to execute the command. Defaults to only container if there is only one container in the pod.
    pub container: Option<&'a str>,
    /// Stderr if true indicates that stderr is to be redirected for the attach call. Defaults to true.
    pub stderr: Option<bool>,
    /// Stdin if true, redirects the standard input stream of the pod for this call. Defaults to false.
    pub stdin: Option<bool>,
    /// Stdout if true indicates that stdout is to be redirected for the attach call. Defaults to true.
    pub stdout: Option<bool>,
    /// TTY if true indicates that a tty will be allocated for the attach call. This is passed through the container runtime so the tty is allocated on the worker node by the container runtime. Defaults to false.
    pub tty: Option<bool>,
}

// Generated from operation connectCoreV1GetNamespacedPodExec

impl Pod {
    /// connect GET requests to exec of Pod
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the PodExecOptions
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn connect_get_exec(
        name: &str,
        namespace: &str,
        optional: ConnectGetPodExecOptional<'_>,
    ) -> Result<crate::http::Request<Vec<u8>>, crate::RequestError> {
        let ConnectGetPodExecOptional {
            command,
            container,
            stderr,
            stdin,
            stdout,
            tty,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/exec?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        if let Some(command) = command {
            for command in command {
                __query_pairs.append_pair("command", command);
            }
        }
        if let Some(container) = container {
            __query_pairs.append_pair("container", container);
        }
        if let Some(stderr) = stderr {
            __query_pairs.append_pair("stderr", if stderr { "true" } else { "false" });
        }
        if let Some(stdin) = stdin {
            __query_pairs.append_pair("stdin", if stdin { "true" } else { "false" });
        }
        if let Some(stdout) = stdout {
            __query_pairs.append_pair("stdout", if stdout { "true" } else { "false" });
        }
        if let Some(tty) = tty {
            __query_pairs.append_pair("tty", if tty { "true" } else { "false" });
        }
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`Pod::connect_get_exec`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectGetPodExecOptional<'a> {
    /// Command is the remote command to execute. argv array. Not executed within a shell.
    pub command: Option<&'a [String]>,
    /// Container in which to execute the command. Defaults to only container if there is only one container in the pod.
    pub container: Option<&'a str>,
    /// Redirect the standard error stream of the pod for this call. Defaults to true.
    pub stderr: Option<bool>,
    /// Redirect the standard input stream of the pod for this call. Defaults to false.
    pub stdin: Option<bool>,
    /// Redirect the standard output stream of the pod for this call. Defaults to true.
    pub stdout: Option<bool>,
    /// TTY if true indicates that a tty will be allocated for the exec call. Defaults to false.
    pub tty: Option<bool>,
}

// Generated from operation connectCoreV1GetNamespacedPodPortforward

impl Pod {
    /// connect GET requests to portforward of Pod
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the PodPortForwardOptions
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn connect_get_portforward(
        name: &str,
        namespace: &str,
        optional: ConnectGetPodPortforwardOptional,
    ) -> Result<crate::http::Request<Vec<u8>>, crate::RequestError> {
        let ConnectGetPodPortforwardOptional {
            ports,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/portforward?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        if let Some(ports) = ports {
            __query_pairs.append_pair("ports", &ports.to_string());
        }
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`Pod::connect_get_portforward`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectGetPodPortforwardOptional {
    /// List of ports to forward Required when using WebSockets
    pub ports: Option<i64>,
}

// Generated from operation connectCoreV1GetNamespacedPodProxy

impl Pod {
    /// connect GET requests to proxy of Pod
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the PodProxyOptions
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn connect_get_proxy(
        name: &str,
        namespace: &str,
        optional: ConnectGetPodProxyOptional<'_>,
    ) -> Result<crate::http::Request<Vec<u8>>, crate::RequestError> {
        let ConnectGetPodProxyOptional {
            path,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/proxy?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        if let Some(path) = path {
            __query_pairs.append_pair("path", path);
        }
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`Pod::connect_get_proxy`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectGetPodProxyOptional<'a> {
    /// Path is the URL path to use for the current proxy request to pod.
    pub path: Option<&'a str>,
}

// Generated from operation connectCoreV1GetNamespacedPodProxyWithPath

impl Pod {
    /// connect GET requests to proxy of Pod
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the PodProxyOptions
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `path`
    ///
    ///     path to the resource
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn connect_get_proxy_with_path(
        name: &str,
        namespace: &str,
        path: &str,
        optional: ConnectGetPodProxyWithPathOptional<'_>,
    ) -> Result<crate::http::Request<Vec<u8>>, crate::RequestError> {
        let ConnectGetPodProxyWithPathOptional {
            path_,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/proxy/{path}?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            path = crate::percent_encoding::percent_encode(path.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        if let Some(path_) = path_ {
            __query_pairs.append_pair("path", path_);
        }
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`Pod::connect_get_proxy_with_path`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectGetPodProxyWithPathOptional<'a> {
    /// Path is the URL path to use for the current proxy request to pod.
    pub path_: Option<&'a str>,
}

// Generated from operation connectCoreV1PatchNamespacedPodProxy

impl Pod {
    /// connect PATCH requests to proxy of Pod
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the PodProxyOptions
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn connect_patch_proxy(
        name: &str,
        namespace: &str,
        optional: ConnectPatchPodProxyOptional<'_>,
    ) -> Result<crate::http::Request<Vec<u8>>, crate::RequestError> {
        let ConnectPatchPodProxyOptional {
            path,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/proxy?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        if let Some(path) = path {
            __query_pairs.append_pair("path", path);
        }
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::patch(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`Pod::connect_patch_proxy`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectPatchPodProxyOptional<'a> {
    /// Path is the URL path to use for the current proxy request to pod.
    pub path: Option<&'a str>,
}

// Generated from operation connectCoreV1PatchNamespacedPodProxyWithPath

impl Pod {
    /// connect PATCH requests to proxy of Pod
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the PodProxyOptions
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `path`
    ///
    ///     path to the resource
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn connect_patch_proxy_with_path(
        name: &str,
        namespace: &str,
        path: &str,
        optional: ConnectPatchPodProxyWithPathOptional<'_>,
    ) -> Result<crate::http::Request<Vec<u8>>, crate::RequestError> {
        let ConnectPatchPodProxyWithPathOptional {
            path_,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/proxy/{path}?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            path = crate::percent_encoding::percent_encode(path.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        if let Some(path_) = path_ {
            __query_pairs.append_pair("path", path_);
        }
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::patch(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`Pod::connect_patch_proxy_with_path`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectPatchPodProxyWithPathOptional<'a> {
    /// Path is the URL path to use for the current proxy request to pod.
    pub path_: Option<&'a str>,
}

// Generated from operation connectCoreV1PostNamespacedPodAttach

impl Pod {
    /// connect POST requests to attach of Pod
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the PodAttachOptions
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn connect_post_attach(
        name: &str,
        namespace: &str,
        optional: ConnectPostPodAttachOptional<'_>,
    ) -> Result<crate::http::Request<Vec<u8>>, crate::RequestError> {
        let ConnectPostPodAttachOptional {
            container,
            stderr,
            stdin,
            stdout,
            tty,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/attach?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        if let Some(container) = container {
            __query_pairs.append_pair("container", container);
        }
        if let Some(stderr) = stderr {
            __query_pairs.append_pair("stderr", if stderr { "true" } else { "false" });
        }
        if let Some(stdin) = stdin {
            __query_pairs.append_pair("stdin", if stdin { "true" } else { "false" });
        }
        if let Some(stdout) = stdout {
            __query_pairs.append_pair("stdout", if stdout { "true" } else { "false" });
        }
        if let Some(tty) = tty {
            __query_pairs.append_pair("tty", if tty { "true" } else { "false" });
        }
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::post(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`Pod::connect_post_attach`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectPostPodAttachOptional<'a> {
    /// The container in which to execute the command. Defaults to only container if there is only one container in the pod.
    pub container: Option<&'a str>,
    /// Stderr if true indicates that stderr is to be redirected for the attach call. Defaults to true.
    pub stderr: Option<bool>,
    /// Stdin if true, redirects the standard input stream of the pod for this call. Defaults to false.
    pub stdin: Option<bool>,
    /// Stdout if true indicates that stdout is to be redirected for the attach call. Defaults to true.
    pub stdout: Option<bool>,
    /// TTY if true indicates that a tty will be allocated for the attach call. This is passed through the container runtime so the tty is allocated on the worker node by the container runtime. Defaults to false.
    pub tty: Option<bool>,
}

// Generated from operation connectCoreV1PostNamespacedPodExec

impl Pod {
    /// connect POST requests to exec of Pod
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the PodExecOptions
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn connect_post_exec(
        name: &str,
        namespace: &str,
        optional: ConnectPostPodExecOptional<'_>,
    ) -> Result<crate::http::Request<Vec<u8>>, crate::RequestError> {
        let ConnectPostPodExecOptional {
            command,
            container,
            stderr,
            stdin,
            stdout,
            tty,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/exec?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        if let Some(command) = command {
            __query_pairs.append_pair("command", command);
        }
        if let Some(container) = container {
            __query_pairs.append_pair("container", container);
        }
        if let Some(stderr) = stderr {
            __query_pairs.append_pair("stderr", if stderr { "true" } else { "false" });
        }
        if let Some(stdin) = stdin {
            __query_pairs.append_pair("stdin", if stdin { "true" } else { "false" });
        }
        if let Some(stdout) = stdout {
            __query_pairs.append_pair("stdout", if stdout { "true" } else { "false" });
        }
        if let Some(tty) = tty {
            __query_pairs.append_pair("tty", if tty { "true" } else { "false" });
        }
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::post(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`Pod::connect_post_exec`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectPostPodExecOptional<'a> {
    /// Command is the remote command to execute. argv array. Not executed within a shell.
    pub command: Option<&'a str>,
    /// Container in which to execute the command. Defaults to only container if there is only one container in the pod.
    pub container: Option<&'a str>,
    /// Redirect the standard error stream of the pod for this call. Defaults to true.
    pub stderr: Option<bool>,
    /// Redirect the standard input stream of the pod for this call. Defaults to false.
    pub stdin: Option<bool>,
    /// Redirect the standard output stream of the pod for this call. Defaults to true.
    pub stdout: Option<bool>,
    /// TTY if true indicates that a tty will be allocated for the exec call. Defaults to false.
    pub tty: Option<bool>,
}

// Generated from operation connectCoreV1PostNamespacedPodPortforward

impl Pod {
    /// connect POST requests to portforward of Pod
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the PodPortForwardOptions
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn connect_post_portforward(
        name: &str,
        namespace: &str,
        optional: ConnectPostPodPortforwardOptional,
    ) -> Result<crate::http::Request<Vec<u8>>, crate::RequestError> {
        let ConnectPostPodPortforwardOptional {
            ports,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/portforward?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        if let Some(ports) = ports {
            __query_pairs.append_pair("ports", &ports.to_string());
        }
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::post(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`Pod::connect_post_portforward`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectPostPodPortforwardOptional {
    /// List of ports to forward Required when using WebSockets
    pub ports: Option<i64>,
}

// Generated from operation connectCoreV1PostNamespacedPodProxy

impl Pod {
    /// connect POST requests to proxy of Pod
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the PodProxyOptions
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn connect_post_proxy(
        name: &str,
        namespace: &str,
        optional: ConnectPostPodProxyOptional<'_>,
    ) -> Result<crate::http::Request<Vec<u8>>, crate::RequestError> {
        let ConnectPostPodProxyOptional {
            path,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/proxy?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        if let Some(path) = path {
            __query_pairs.append_pair("path", path);
        }
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::post(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`Pod::connect_post_proxy`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectPostPodProxyOptional<'a> {
    /// Path is the URL path to use for the current proxy request to pod.
    pub path: Option<&'a str>,
}

// Generated from operation connectCoreV1PostNamespacedPodProxyWithPath

impl Pod {
    /// connect POST requests to proxy of Pod
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the PodProxyOptions
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `path`
    ///
    ///     path to the resource
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn connect_post_proxy_with_path(
        name: &str,
        namespace: &str,
        path: &str,
        optional: ConnectPostPodProxyWithPathOptional<'_>,
    ) -> Result<crate::http::Request<Vec<u8>>, crate::RequestError> {
        let ConnectPostPodProxyWithPathOptional {
            path_,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/proxy/{path}?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            path = crate::percent_encoding::percent_encode(path.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        if let Some(path_) = path_ {
            __query_pairs.append_pair("path", path_);
        }
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::post(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`Pod::connect_post_proxy_with_path`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectPostPodProxyWithPathOptional<'a> {
    /// Path is the URL path to use for the current proxy request to pod.
    pub path_: Option<&'a str>,
}

// Generated from operation connectCoreV1PutNamespacedPodProxy

impl Pod {
    /// connect PUT requests to proxy of Pod
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the PodProxyOptions
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn connect_put_proxy(
        name: &str,
        namespace: &str,
        optional: ConnectPutPodProxyOptional<'_>,
    ) -> Result<crate::http::Request<Vec<u8>>, crate::RequestError> {
        let ConnectPutPodProxyOptional {
            path,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/proxy?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        if let Some(path) = path {
            __query_pairs.append_pair("path", path);
        }
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::put(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`Pod::connect_put_proxy`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectPutPodProxyOptional<'a> {
    /// Path is the URL path to use for the current proxy request to pod.
    pub path: Option<&'a str>,
}

// Generated from operation connectCoreV1PutNamespacedPodProxyWithPath

impl Pod {
    /// connect PUT requests to proxy of Pod
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the PodProxyOptions
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `path`
    ///
    ///     path to the resource
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn connect_put_proxy_with_path(
        name: &str,
        namespace: &str,
        path: &str,
        optional: ConnectPutPodProxyWithPathOptional<'_>,
    ) -> Result<crate::http::Request<Vec<u8>>, crate::RequestError> {
        let ConnectPutPodProxyWithPathOptional {
            path_,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/proxy/{path}?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            path = crate::percent_encoding::percent_encode(path.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        if let Some(path_) = path_ {
            __query_pairs.append_pair("path", path_);
        }
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::put(__url);
        let __body = vec![];
        __request.body(__body).map_err(crate::RequestError::Http)
    }
}

/// Optional parameters of [`Pod::connect_put_proxy_with_path`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectPutPodProxyWithPathOptional<'a> {
    /// Path is the URL path to use for the current proxy request to pod.
    pub path_: Option<&'a str>,
}

// Generated from operation createCoreV1NamespacedPod

impl Pod {
    /// create a Pod
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`crate::CreateResponse`]`<Self>>` constructor, or [`crate::CreateResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn create(
        namespace: &str,
        body: &crate::api::core::v1::Pod,
        optional: crate::CreateOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<crate::CreateResponse<Self>>), crate::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods?",
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::post(__url);
        let __body = crate::serde_json::to_vec(body).map_err(crate::RequestError::Json)?;
        let __request = __request.header(crate::http::header::CONTENT_TYPE, crate::http::header::HeaderValue::from_static("application/json"));
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

// Generated from operation deleteCoreV1CollectionNamespacedPod

impl Pod {
    /// delete collection of Pod
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`crate::DeleteResponse`]`<`[`crate::List`]`<Self>>>` constructor, or [`crate::DeleteResponse`]`<`[`crate::List`]`<Self>>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `delete_optional`
    ///
    ///     Delete options. Use `Default::default()` to not pass any.
    ///
    /// * `list_optional`
    ///
    ///     List options. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn delete_collection(
        namespace: &str,
        delete_optional: crate::DeleteOptional<'_>,
        list_optional: crate::ListOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<crate::DeleteResponse<crate::List<Self>>>), crate::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods?",
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        list_optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::delete(__url);
        let __body = crate::serde_json::to_vec(&delete_optional).map_err(crate::RequestError::Json)?;
        let __request = __request.header(crate::http::header::CONTENT_TYPE, crate::http::header::HeaderValue::from_static("application/json"));
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

// Generated from operation deleteCoreV1NamespacedPod

impl Pod {
    /// delete a Pod
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`crate::DeleteResponse`]`<Self>>` constructor, or [`crate::DeleteResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Pod
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn delete(
        name: &str,
        namespace: &str,
        optional: crate::DeleteOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<crate::DeleteResponse<Self>>), crate::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );

        let __request = crate::http::Request::delete(__url);
        let __body = crate::serde_json::to_vec(&optional).map_err(crate::RequestError::Json)?;
        let __request = __request.header(crate::http::header::CONTENT_TYPE, crate::http::header::HeaderValue::from_static("application/json"));
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

// Generated from operation listCoreV1NamespacedPod

impl Pod {
    /// list or watch objects of kind Pod
    ///
    /// This operation only supports listing all items of this type.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`crate::ListResponse`]`<Self>>` constructor, or [`crate::ListResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn list(
        namespace: &str,
        optional: crate::ListOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<crate::ListResponse<Self>>), crate::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods?",
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::get(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

// Generated from operation listCoreV1PodForAllNamespaces

impl Pod {
    /// list or watch objects of kind Pod
    ///
    /// This operation only supports listing all items of this type.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`crate::ListResponse`]`<Self>>` constructor, or [`crate::ListResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn list_for_all_namespaces(
        optional: crate::ListOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<crate::ListResponse<Self>>), crate::RequestError> {
        let __url = "/api/v1/pods?".to_owned();
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::get(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

// Generated from operation patchCoreV1NamespacedPod

impl Pod {
    /// partially update the specified Pod
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`crate::PatchResponse`]`<Self>>` constructor, or [`crate::PatchResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Pod
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn patch(
        name: &str,
        namespace: &str,
        body: &crate::apimachinery::pkg::apis::meta::v1::Patch,
        optional: crate::PatchOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<crate::PatchResponse<Self>>), crate::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::patch(__url);
        let __body = crate::serde_json::to_vec(body).map_err(crate::RequestError::Json)?;
        let __request = __request.header(crate::http::header::CONTENT_TYPE, crate::http::header::HeaderValue::from_static(match body {
            crate::apimachinery::pkg::apis::meta::v1::Patch::Json(_) => "application/json-patch+json",
            crate::apimachinery::pkg::apis::meta::v1::Patch::Merge(_) => "application/merge-patch+json",
            crate::apimachinery::pkg::apis::meta::v1::Patch::StrategicMerge(_) => "application/strategic-merge-patch+json",
        }));
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

// Generated from operation patchCoreV1NamespacedPodEphemeralcontainers

impl Pod {
    /// partially update ephemeralcontainers of the specified Pod
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`crate::PatchResponse`]`<Self>>` constructor, or [`crate::PatchResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Pod
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn patch_ephemeralcontainers(
        name: &str,
        namespace: &str,
        body: &crate::apimachinery::pkg::apis::meta::v1::Patch,
        optional: crate::PatchOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<crate::PatchResponse<Self>>), crate::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/ephemeralcontainers?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::patch(__url);
        let __body = crate::serde_json::to_vec(body).map_err(crate::RequestError::Json)?;
        let __request = __request.header(crate::http::header::CONTENT_TYPE, crate::http::header::HeaderValue::from_static(match body {
            crate::apimachinery::pkg::apis::meta::v1::Patch::Json(_) => "application/json-patch+json",
            crate::apimachinery::pkg::apis::meta::v1::Patch::Merge(_) => "application/merge-patch+json",
            crate::apimachinery::pkg::apis::meta::v1::Patch::StrategicMerge(_) => "application/strategic-merge-patch+json",
        }));
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

// Generated from operation patchCoreV1NamespacedPodStatus

impl Pod {
    /// partially update status of the specified Pod
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`crate::PatchResponse`]`<Self>>` constructor, or [`crate::PatchResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Pod
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn patch_status(
        name: &str,
        namespace: &str,
        body: &crate::apimachinery::pkg::apis::meta::v1::Patch,
        optional: crate::PatchOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<crate::PatchResponse<Self>>), crate::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/status?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::patch(__url);
        let __body = crate::serde_json::to_vec(body).map_err(crate::RequestError::Json)?;
        let __request = __request.header(crate::http::header::CONTENT_TYPE, crate::http::header::HeaderValue::from_static(match body {
            crate::apimachinery::pkg::apis::meta::v1::Patch::Json(_) => "application/json-patch+json",
            crate::apimachinery::pkg::apis::meta::v1::Patch::Merge(_) => "application/merge-patch+json",
            crate::apimachinery::pkg::apis::meta::v1::Patch::StrategicMerge(_) => "application/strategic-merge-patch+json",
        }));
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

// Generated from operation readCoreV1NamespacedPod

impl Pod {
    /// read the specified Pod
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReadPodResponse`]`>` constructor, or [`ReadPodResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Pod
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    #[cfg(feature = "api")]
    pub fn read(
        name: &str,
        namespace: &str,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<ReadPodResponse>), crate::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );

        let __request = crate::http::Request::get(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Use `<ReadPodResponse as Response>::try_from_parts` to parse the HTTP response body of [`Pod::read`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ReadPodResponse {
    Ok(crate::api::core::v1::Pod),
    Other(Result<Option<crate::serde_json::Value>, crate::serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for ReadPodResponse {
    fn try_from_parts(status_code: crate::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            crate::http::StatusCode::OK => {
                let result = match crate::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadPodResponse::Ok(result), buf.len()))
            },
            _ => {
                let (result, read) =
                    if buf.is_empty() {
                        (Ok(None), 0)
                    }
                    else {
                        match crate::serde_json::from_slice(buf) {
                            Ok(value) => (Ok(Some(value)), buf.len()),
                            Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                            Err(err) => (Err(err), 0),
                        }
                    };
                Ok((ReadPodResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation readCoreV1NamespacedPodEphemeralcontainers

impl Pod {
    /// read ephemeralcontainers of the specified Pod
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReadPodEphemeralcontainersResponse`]`>` constructor, or [`ReadPodEphemeralcontainersResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Pod
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    #[cfg(feature = "api")]
    pub fn read_ephemeralcontainers(
        name: &str,
        namespace: &str,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<ReadPodEphemeralcontainersResponse>), crate::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/ephemeralcontainers",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );

        let __request = crate::http::Request::get(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Use `<ReadPodEphemeralcontainersResponse as Response>::try_from_parts` to parse the HTTP response body of [`Pod::read_ephemeralcontainers`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ReadPodEphemeralcontainersResponse {
    Ok(crate::api::core::v1::Pod),
    Other(Result<Option<crate::serde_json::Value>, crate::serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for ReadPodEphemeralcontainersResponse {
    fn try_from_parts(status_code: crate::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            crate::http::StatusCode::OK => {
                let result = match crate::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadPodEphemeralcontainersResponse::Ok(result), buf.len()))
            },
            _ => {
                let (result, read) =
                    if buf.is_empty() {
                        (Ok(None), 0)
                    }
                    else {
                        match crate::serde_json::from_slice(buf) {
                            Ok(value) => (Ok(Some(value)), buf.len()),
                            Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                            Err(err) => (Err(err), 0),
                        }
                    };
                Ok((ReadPodEphemeralcontainersResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation readCoreV1NamespacedPodLog

impl Pod {
    /// read log of the specified Pod
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReadPodLogResponse`]`>` constructor, or [`ReadPodLogResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Pod
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn read_log(
        name: &str,
        namespace: &str,
        optional: ReadPodLogOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<ReadPodLogResponse>), crate::RequestError> {
        let ReadPodLogOptional {
            container,
            follow,
            insecure_skip_tls_verify_backend,
            limit_bytes,
            previous,
            since_seconds,
            tail_lines,
            timestamps,
        } = optional;
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/log?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        if let Some(container) = container {
            __query_pairs.append_pair("container", container);
        }
        if let Some(follow) = follow {
            __query_pairs.append_pair("follow", if follow { "true" } else { "false" });
        }
        if let Some(insecure_skip_tls_verify_backend) = insecure_skip_tls_verify_backend {
            __query_pairs.append_pair("insecureSkipTLSVerifyBackend", if insecure_skip_tls_verify_backend { "true" } else { "false" });
        }
        if let Some(limit_bytes) = limit_bytes {
            __query_pairs.append_pair("limitBytes", &limit_bytes.to_string());
        }
        if let Some(previous) = previous {
            __query_pairs.append_pair("previous", if previous { "true" } else { "false" });
        }
        if let Some(since_seconds) = since_seconds {
            __query_pairs.append_pair("sinceSeconds", &since_seconds.to_string());
        }
        if let Some(tail_lines) = tail_lines {
            __query_pairs.append_pair("tailLines", &tail_lines.to_string());
        }
        if let Some(timestamps) = timestamps {
            __query_pairs.append_pair("timestamps", if timestamps { "true" } else { "false" });
        }
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::get(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Optional parameters of [`Pod::read_log`]
#[cfg(feature = "api")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReadPodLogOptional<'a> {
    /// The container for which to stream logs. Defaults to only container if there is one container in the pod.
    pub container: Option<&'a str>,
    /// Follow the log stream of the pod. Defaults to false.
    pub follow: Option<bool>,
    /// insecureSkipTLSVerifyBackend indicates that the apiserver should not confirm the validity of the serving certificate of the backend it is connecting to.  This will make the HTTPS connection between the apiserver and the backend insecure. This means the apiserver cannot verify the log data it is receiving came from the real kubelet.  If the kubelet is configured to verify the apiserver's TLS credentials, it does not mean the connection to the real kubelet is vulnerable to a man in the middle attack (e.g. an attacker could not intercept the actual log data coming from the real kubelet).
    pub insecure_skip_tls_verify_backend: Option<bool>,
    /// If set, the number of bytes to read from the server before terminating the log output. This may not display a complete final line of logging, and may return slightly more or slightly less than the specified limit.
    pub limit_bytes: Option<i64>,
    /// Return previous terminated container logs. Defaults to false.
    pub previous: Option<bool>,
    /// A relative time in seconds before the current time from which to show logs. If this value precedes the time a pod was started, only logs since the pod start will be returned. If this value is in the future, no logs will be returned. Only one of sinceSeconds or sinceTime may be specified.
    pub since_seconds: Option<i64>,
    /// If set, the number of lines from the end of the logs to show. If not specified, logs are shown from the creation of the container or sinceSeconds or sinceTime
    pub tail_lines: Option<i64>,
    /// If true, add an RFC3339 or RFC3339Nano timestamp at the beginning of every line of log output. Defaults to false.
    pub timestamps: Option<bool>,
}

/// Use `<ReadPodLogResponse as Response>::try_from_parts` to parse the HTTP response body of [`Pod::read_log`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ReadPodLogResponse {
    Ok(String),
    Other(Result<Option<crate::serde_json::Value>, crate::serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for ReadPodLogResponse {
    fn try_from_parts(status_code: crate::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            crate::http::StatusCode::OK => {
                if buf.is_empty() {
                    return Err(crate::ResponseError::NeedMoreData);
                }

                let (result, len) = match std::str::from_utf8(buf) {
                    Ok(s) => (s, buf.len()),
                    Err(err) => match (err.valid_up_to(), err.error_len()) {
                        (0, Some(_)) => return Err(crate::ResponseError::Utf8(err)),
                        (0, None) => return Err(crate::ResponseError::NeedMoreData),
                        (valid_up_to, _) => (
                            unsafe { std::str::from_utf8_unchecked(buf.get_unchecked(..valid_up_to)) },
                            valid_up_to,
                        ),
                    },
                };
                Ok((ReadPodLogResponse::Ok(result.to_owned()), len))
            },
            _ => {
                let (result, read) =
                    if buf.is_empty() {
                        (Ok(None), 0)
                    }
                    else {
                        match crate::serde_json::from_slice(buf) {
                            Ok(value) => (Ok(Some(value)), buf.len()),
                            Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                            Err(err) => (Err(err), 0),
                        }
                    };
                Ok((ReadPodLogResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation readCoreV1NamespacedPodStatus

impl Pod {
    /// read status of the specified Pod
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`ReadPodStatusResponse`]`>` constructor, or [`ReadPodStatusResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Pod
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    #[cfg(feature = "api")]
    pub fn read_status(
        name: &str,
        namespace: &str,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<ReadPodStatusResponse>), crate::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/status",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );

        let __request = crate::http::Request::get(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Use `<ReadPodStatusResponse as Response>::try_from_parts` to parse the HTTP response body of [`Pod::read_status`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum ReadPodStatusResponse {
    Ok(crate::api::core::v1::Pod),
    Other(Result<Option<crate::serde_json::Value>, crate::serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for ReadPodStatusResponse {
    fn try_from_parts(status_code: crate::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            crate::http::StatusCode::OK => {
                let result = match crate::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((ReadPodStatusResponse::Ok(result), buf.len()))
            },
            _ => {
                let (result, read) =
                    if buf.is_empty() {
                        (Ok(None), 0)
                    }
                    else {
                        match crate::serde_json::from_slice(buf) {
                            Ok(value) => (Ok(Some(value)), buf.len()),
                            Err(err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                            Err(err) => (Err(err), 0),
                        }
                    };
                Ok((ReadPodStatusResponse::Other(result), read))
            },
        }
    }
}

// Generated from operation replaceCoreV1NamespacedPod

impl Pod {
    /// replace the specified Pod
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`crate::ReplaceResponse`]`<Self>>` constructor, or [`crate::ReplaceResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Pod
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn replace(
        name: &str,
        namespace: &str,
        body: &crate::api::core::v1::Pod,
        optional: crate::ReplaceOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<crate::ReplaceResponse<Self>>), crate::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::put(__url);
        let __body = crate::serde_json::to_vec(body).map_err(crate::RequestError::Json)?;
        let __request = __request.header(crate::http::header::CONTENT_TYPE, crate::http::header::HeaderValue::from_static("application/json"));
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

// Generated from operation replaceCoreV1NamespacedPodEphemeralcontainers

impl Pod {
    /// replace ephemeralcontainers of the specified Pod
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`crate::ReplaceResponse`]`<Self>>` constructor, or [`crate::ReplaceResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Pod
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn replace_ephemeralcontainers(
        name: &str,
        namespace: &str,
        body: &crate::api::core::v1::Pod,
        optional: crate::ReplaceOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<crate::ReplaceResponse<Self>>), crate::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/ephemeralcontainers?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::put(__url);
        let __body = crate::serde_json::to_vec(body).map_err(crate::RequestError::Json)?;
        let __request = __request.header(crate::http::header::CONTENT_TYPE, crate::http::header::HeaderValue::from_static("application/json"));
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

// Generated from operation replaceCoreV1NamespacedPodStatus

impl Pod {
    /// replace status of the specified Pod
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`crate::ReplaceResponse`]`<Self>>` constructor, or [`crate::ReplaceResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the Pod
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn replace_status(
        name: &str,
        namespace: &str,
        body: &crate::api::core::v1::Pod,
        optional: crate::ReplaceOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<crate::ReplaceResponse<Self>>), crate::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/status?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::put(__url);
        let __body = crate::serde_json::to_vec(body).map_err(crate::RequestError::Json)?;
        let __request = __request.header(crate::http::header::CONTENT_TYPE, crate::http::header::HeaderValue::from_static("application/json"));
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

// Generated from operation watchCoreV1NamespacedPod

impl Pod {
    /// list or watch objects of kind Pod
    ///
    /// This operation only supports watching one item, or a list of items, of this type for changes.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`crate::WatchResponse`]`<Self>>` constructor, or [`crate::WatchResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn watch(
        namespace: &str,
        optional: crate::WatchOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<crate::WatchResponse<Self>>), crate::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods?",
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::get(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

// Generated from operation watchCoreV1PodForAllNamespaces

impl Pod {
    /// list or watch objects of kind Pod
    ///
    /// This operation only supports watching one item, or a list of items, of this type for changes.
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`crate::WatchResponse`]`<Self>>` constructor, or [`crate::WatchResponse`]`<Self>` directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn watch_for_all_namespaces(
        optional: crate::WatchOptional<'_>,
    ) -> Result<(crate::http::Request<Vec<u8>>, fn(crate::http::StatusCode) -> crate::ResponseBody<crate::WatchResponse<Self>>), crate::RequestError> {
        let __url = "/api/v1/pods?".to_owned();
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = crate::http::Request::get(__url);
        let __body = vec![];
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

// End /v1/Pod

impl crate::Resource for Pod {
    const API_VERSION: &'static str = "v1";
    const GROUP: &'static str = "";
    const KIND: &'static str = "Pod";
    const VERSION: &'static str = "v1";
    const URL_PATH_SEGMENT: &'static str = "pods";
    type Scope = crate::NamespaceResourceScope;
}

impl crate::ListableResource for Pod {
    const LIST_KIND: &'static str = "PodList";
}

impl crate::Metadata for Pod {
    type Ty = crate::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> &<Self as crate::Metadata>::Ty {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut<Self as crate::Metadata>::Ty {
        &mut self.metadata
    }
}

impl<'de> crate::serde::Deserialize<'de> for Pod {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_metadata,
            Key_spec,
            Key_status,
            Other,
        }

        impl<'de> crate::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> crate::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: crate::serde::de::Error {
                        Ok(match v {
                            "apiVersion" => Field::Key_api_version,
                            "kind" => Field::Key_kind,
                            "metadata" => Field::Key_metadata,
                            "spec" => Field::Key_spec,
                            "status" => Field::Key_status,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = Pod;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(<Self::Value as crate::Resource>::KIND)
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_metadata: Option<crate::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<crate::api::core::v1::PodSpec> = None;
                let mut value_status: Option<crate::api::core::v1::PodStatus> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => {
                            let value_api_version: String = crate::serde::de::MapAccess::next_value(&mut map)?;
                            if value_api_version != <Self::Value as crate::Resource>::API_VERSION {
                                return Err(crate::serde::de::Error::invalid_value(crate::serde::de::Unexpected::Str(&value_api_version), &<Self::Value as crate::Resource>::API_VERSION));
                            }
                        },
                        Field::Key_kind => {
                            let value_kind: String = crate::serde::de::MapAccess::next_value(&mut map)?;
                            if value_kind != <Self::Value as crate::Resource>::KIND {
                                return Err(crate::serde::de::Error::invalid_value(crate::serde::de::Unexpected::Str(&value_kind), &<Self::Value as crate::Resource>::KIND));
                            }
                        },
                        Field::Key_metadata => value_metadata = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_spec => value_spec = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_status => value_status = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Pod {
                    metadata: value_metadata.unwrap_or_default(),
                    spec: value_spec,
                    status: value_status,
                })
            }
        }

        deserializer.deserialize_struct(
            <Self as crate::Resource>::KIND,
            &[
                "apiVersion",
                "kind",
                "metadata",
                "spec",
                "status",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for Pod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            <Self as crate::Resource>::KIND,
            3 +
            self.spec.as_ref().map_or(0, |_| 1) +
            self.status.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as crate::Resource>::API_VERSION)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as crate::Resource>::KIND)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", &self.metadata)?;
        if let Some(value) = &self.spec {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "spec", value)?;
        }
        if let Some(value) = &self.status {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "status", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for Pod {
    fn schema_name() -> String {
        "io.k8s.api.core.v1.Pod".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("Pod is a collection of containers that can run on a host. This resource is created by clients and scheduled onto hosts.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "apiVersion".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "kind".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "metadata".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::ObjectMeta>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "spec".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::PodSpec>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Specification of the desired behavior of the pod. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "status".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::core::v1::PodStatus>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Most recently observed status of the pod. This data may not be up to date. Populated by the system. Read-only. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                required: [
                    "metadata".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
