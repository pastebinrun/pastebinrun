INSERT INTO pastes (identifier, language_id, paste)
    SELECT
        'about',
        language_id,
        '# About

This is a free (as-in-freedom) pastebin software written in Rust, you can check out its [AGPLv3](https://www.gnu.org/licenses/agpl-3.0.html)-or-later source code at <https://gitlab.com/pastebin.run/server>. If you have any feedback, feel free to contact me on `pastebinrun -at- borowski -dot- pw`.

## Privacy policy

Last update: March 14, 2019

We store your IP addresses in logs for security purposes only. If you don''t want your IP address to be recorded, please use the [Tor Browser](https://www.torproject.org/) to visit this website. No other information is stored by us. In event a paste on our website contains personal information, please contact us at `pastebinrun -at- borowski -dot- pw` for paste removal purposes.

This website doesn''t publish a list of submitted pastes, and URLs are long enough to be unlikely to randomly guess - 10 characters from an alphabet of 48 characters, which gives the probability of guessing an URL of about 1.5e-17. The paste you are reading is an exception to this rule.

Technical details: We are currently using [HC-128 stream cipher](https://en.wikipedia.org/wiki/HC-256) provided by [Rust''s rand crate](https://crates.io/crates/rand) for generating random paste URLs, with /dev/urandom being used as a random generator.

We are using Cloudflare ([Cloudflare''s privacy policy](https://www.cloudflare.com/privacypolicy/)) and Linode ([Linode''s privacy policy](https://www.linode.com/privacy)) for hosting purposes. Cloudflare acts as a proxy and can read and modify all requests and responses as well as set their own cookies such as `__cfuid` used for its security functionality. You can disable those cookies in web browser, but it may make Cloudflare blocks more likely.

## Tips

The server costs about $5 monthly for Linode 1GB hosting and $8 yearly for a domain. If you want to help cover the costs (I don''t know why you would, but hey), feel free to tip using cryptocurrencies. Just a word of warning: those addresses are reused, if you have any suggestions on how to avoid that, feel free to propose those.

Bitcoin: 34W7QXybvoSxKkcsNWQkDZ3iUzztxvGchq

[![](/static/qr/bitcoin.png)](bitcoin:34W7QXybvoSxKkcsNWQkDZ3iUzztxvGchq)

Monero: 47sPotFZMWxWAvmZp6VCKXYN4pzoWtJKC1DpCYDw9JLmfm9gL8YsXqULBbcQrB2K2JDQmQKdLawAWLTUjEqWZDvu3z69tpX

![](/static/qr/monero.png)'
    FROM
        languages
    WHERE
        languages.name = 'Markdown';
