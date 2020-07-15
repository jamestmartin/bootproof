# Network
Protocols I need to support for a working network stack.

## Minimum Viable Product
* Link layer: Ethernet
* Internet layer: IPv6, ICMPv6
* Transport layer: TCP, UDP
* Application layer: DNS, HTTP/1.0

## Very important
* Link layer: 802.11ac, 802.11n, WPA2, WPA3
* Internet layer: IPv4, ICMP, IGMP
* Application layer: DHCP

## Important
* Application layer:
    * TLS:
        * Versions: 1.3
        * Ciphers: AES GCM (128, 256), ChaCha20-Poly1305
        * MACs: AEAD, HMAC-SHA256/384
        * Certificates: RSA, EC
        * Curves: secp512r1, secp384r1, secp256r1, x25519, ed25519, x448, ed448
    * HTTPS
    * IRCS
    * DNS over HTTPS

## Less important
* Application layer:
    * HTTP/2, ALPN
    * TLS 1.2
    * IMAP
    * SMTP
    * SSH

## Unimportant
* Transport layer: QUIC
* Application layer:
    * HTTP/3
* TLS MACs: GOST 28147-89 IMIT, GOST R 34.11-94
* Other: HSTS + preloading, OCSP stapling, certificate transparency, session resumption
