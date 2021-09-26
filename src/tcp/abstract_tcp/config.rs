use crate::altcp::altcp_tls_mbedtls::AlTcpTlsConfig;
use crate::defines::LwipAddr;

pub struct AltcpProxyconnectConfig {
    pub proxy_addr: LwipAddr,
    pub proxy_port: u16,
}

pub struct AltcpProxyconnectTlsConfig {
    proxy: AltcpProxyconnectConfig,
    tls_config: AlTcpTlsConfig,
}
