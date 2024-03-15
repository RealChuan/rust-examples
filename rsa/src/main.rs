mod rsa;
use rsa::{rsa_decrypt, rsa_encrypt};
mod openssl_rsa;
use openssl_rsa::{openssl_rsa_decrypt, openssl_rsa_encrypt};

fn main() {
    let private_key = "-----BEGIN PRIVATE KEY-----
MIIJQwIBADANBgkqhkiG9w0BAQEFAASCCS0wggkpAgEAAoICAQDLA4q2SYNZucWs
RkGT7BgDnXLmK37GmDdPjIemynj+EaqemvKL9ljhLAV+HHmChqp4AfE823CKZjOt
SgG3FW//iJz/vqGgyzga7Bv10CbILb5b1rKlRZWoDoGdqm8QXThwHemWbl5BdvBv
xIW1XuCwR+yn4v90wwbimheag+3GZ2YiVtj501OA1/AKUR9/bInTdmk6wf0LBupf
/lJkcwLWpne6rBGOG6zUH3hEtewUPzWNiYcnCFr83guAPfwtSK7StUvqspjRcRpO
9rW1R6V2O8hVFtTpptZylQw0E5SabKfeOdjuK48EMzz0+pSVomLKj8W661diWF0y
xVQAdf8oAlW+XT6ICCZPdEC5kJHlMC7fk1+2t350DAQeXVGrx4L4jCrpkLbkl7sK
giJlwiBfoKKrtkyX/3632iqxO/cmO2Uk9JNhhlBrB1nmVUWN0ArOHZReQAD0ADgc
9oJbi0zSdWIpVk9hJRkl+CGTkpdcqCx9I+mKzAYUrecajeINorpBv6l28cJPDzRu
Pi1tYmyaOkx+rYKSpUzaTehAAYQfqSzGZUWAG5qoP8AL9Wh+pm0p/u69yC1G/jYi
e5mhCFkLUTJpXsG7kjgEv0DsZaNT0nQlpAPP0DmItwsbTSRKjki2X4SXVmYjzzqP
oa/RRj1B1DcMlsWQBSaNhJyMQ+dPiQIDAQABAoICAQC3vKLkTDBXGq1vN77OVBV/
4BZHfwIkV0tn8bYsHH/Y4sBa/72ZMwJJ+P+XncnTenH6Xzeobqqyd8E5iwt2Wi6d
RVVimXPLBWHn6916YkNXVR8+G1hgAFRsX4Sr377ZPqV/w3LPtWAVO8JsRZ8GAbK1
mML1R5sQYBpxKClIrTKcKKTlW1jQ8pRWvhkAHV+2gdNT0Y13om6UdlPVQWXsh7L/
9nXSOz1AMof/C+Rbu5LI2FRtnxT1HLjMt2INVQxRJ8Eh7vegmLd6hzTaVqsqQIfe
NEkhyaI1PcxEs+TxlO8fvEtJ820EUeQEVyNKTSgBLZDf57N7ctgJqBPhiXp3WFPu
kvyDPYORmJFftKrxAn+ghvFTqH5b8CCHyplxOQD2LU5OuktAGGWDqz8+JDOh7flR
hXwuPYAxaZJZk3xtfuir27Gx9JfA8k8/pSpZc7fai1qBQLPWWd/7O6ShbmFsISKn
yT49ZGrZ36GO6mEKqStjECKWnWHRNoPJRdEjbFnw0DRa9BABfmqVvwF/F3Z7ERrq
CfYXvFTbkjRpixWHCohTwy1gUWZ4rnetrp2HluYsap6zK7ymhVk+ZRhAISIVaBTD
VEA8hfcieh5z+C0jeeSZ/uJNPjCFeV/TH1AveHNNr3RE4I/S1d8aiC6vCue0ju7x
uI8lfewEBrUzNp/TRPxlIQKCAQEA7Y1e1vpesRadCjSDE+0D9o9wa80EVgpAz9KI
m7D8tDNwjFGwWnUgQO/ARasYYgRyPCeu9OblFQuGqs9nRDsu3hVkFA7U9iSo5XrV
f6airZ6PKz7A9wMfLXOjhENAgGY/nIhVJt6Tbfr0AX2YUbpwmbtgkATwlFnLFhkx
wPoIAf27FqiS22uPDHr/Po154pl95sAKeVvHfQ1A8h1svoagO+GwitviEECmdjUr
CBEhIGVPAzSigQNbr+ussbEox8budx4kHcNdzgfLMrg428Rq4KPlAxL2P63cJwiz
CsnGHbBTyrxK4LCSq5r3qmbHlggM0KwCRsL8jW+k8xZg20ibDQKCAQEA2seI7G5x
6uXZAgMA/O40VO9mENp8zLI4upZN4lnTvj1uA7CbrdIeuvb/H2lenYMOakhNHvml
20jWpPWVCOBxIFQE55+I5yV5cuipcsl9klLqujRMggNk61ohv8jIxLFp43faRJ9h
d7WkRxLu21vYrfjJXXdYI0wO/etfrtFd1EySgGK6b5f7R6eDWUUu4XzLSP25hF3M
GicEIVlOD4Uqgddbo8bt4L4K+qdLW69756U8Q6z546nUYua/DSaQ8p9xbsa+MkKe
J+bKsZqP/v1fcn+aVtX9Cu75ibymAo0KPdkHZcLAOagQkXXwVXLnzgrSLeWQakMe
MJcFqL1hZgO3bQKCAQEA5JPUfz144iOlRk2TFxhIaPZKsaAbnQLOrGqjwlQCRQ7+
LDbqX5A2UeWgeCkTFDW9Cf8RTVvjcGWf1jot3qyoA1BH9Tj0WEicL/wK9YKNJEze
7Di9CrH5TQ3peHoR1bsNWHyimv0ulXQVWF3WtEWW1YZEezAVolnoncHuawbrr6bL
YBki/nk3s0Wi1zjg4Fbbm66MJYxXnw+iv5XqrkXyZsxayLKdiw/6XOH6HxAEdRym
c6ow970Ye2VlzbFvURQtH3dkPzNezsIXFx3GsEkGolNyKIwVYN7VaMXpKedPd//U
EM8vqZad+vvpdhDh2Y6zbhMlOZuj0iBscu/EkRwC/QKCAQA+dk+CaTn+/aaKa38h
t4SbZ0o1r5uH+a42n2kXIdxRaUp6pTadVutNJ+3I8xV/VQaNk5zpJg+TXuVhbXgi
c6n4t+Q6mRLGI63b/885JYvDdgbsKLhbOkra4v3VuqaPebOorPRVOUfjnKmNXTty
x3NCoFAlbkhss29oUYYc7EukrEP7gbSG3dZxK8NVfmoXGMEVUtNflkQKh28nck7W
IYf0UB8q0EXkL34E+DSMtbpjdYqSK9WgiUFJB2Yug/9INft1zCpLQTfUXpEqSd2o
yIAyuXnAJxI6x+XYzFJ9UfVFmSwCgqrNC03Hqet3CfpimjZNwkgL/qOk7clA1ujy
7ka1AoIBAGSlXgKGbzC+ZzLzDaYQ+cZNO/hgkTfZkcmVb4MzUt+Bvxsi/yKWuf0E
2K0jhy/XcYH+dbX7MrOkioY/22/qBnOeh/q1oablCpa8UC9mNpJsCDVEii1Gzwr2
+fetpWnJIuQMqhhwYDmgqugHzYuVCpRTT6V77SxvK+ZTtAEEvT6Ydua6JpN9C/xO
BiplsF7wLdfumwk1WItMmRxUQB42g5saXURSQ9y6LQxZHwGQQKG6ADPNALxS0WwU
Aeb/chwf3kkwtMD5J+vFNNh8EfsQVYAKxalb06YZHccgWai+oaxSd09ETnxRbMYJ
SxqiDTmy5mU7ell+gj4JSC1C4F8jELI=
-----END PRIVATE KEY-----";
    let public_key = "-----BEGIN PUBLIC KEY-----
MIICIjANBgkqhkiG9w0BAQEFAAOCAg8AMIICCgKCAgEAywOKtkmDWbnFrEZBk+wY
A51y5it+xpg3T4yHpsp4/hGqnpryi/ZY4SwFfhx5goaqeAHxPNtwimYzrUoBtxVv
/4ic/76hoMs4Guwb9dAmyC2+W9aypUWVqA6BnapvEF04cB3plm5eQXbwb8SFtV7g
sEfsp+L/dMMG4poXmoPtxmdmIlbY+dNTgNfwClEff2yJ03ZpOsH9CwbqX/5SZHMC
1qZ3uqwRjhus1B94RLXsFD81jYmHJwha/N4LgD38LUiu0rVL6rKY0XEaTva1tUel
djvIVRbU6abWcpUMNBOUmmyn3jnY7iuPBDM89PqUlaJiyo/FuutXYlhdMsVUAHX/
KAJVvl0+iAgmT3RAuZCR5TAu35Nftrd+dAwEHl1Rq8eC+Iwq6ZC25Je7CoIiZcIg
X6Ciq7ZMl/9+t9oqsTv3JjtlJPSTYYZQawdZ5lVFjdAKzh2UXkAA9AA4HPaCW4tM
0nViKVZPYSUZJfghk5KXXKgsfSPpiswGFK3nGo3iDaK6Qb+pdvHCTw80bj4tbWJs
mjpMfq2CkqVM2k3oQAGEH6ksxmVFgBuaqD/AC/VofqZtKf7uvcgtRv42InuZoQhZ
C1EyaV7Bu5I4BL9A7GWjU9J0JaQDz9A5iLcLG00kSo5Itl+El1ZmI886j6Gv0UY9
QdQ3DJbFkAUmjYScjEPnT4kCAwEAAQ==
-----END PUBLIC KEY-----";

    let data = "password.";
    {
        let encrypted = rsa_encrypt(public_key, data);
        // println!("Encrypted: {}", encrypted);
        let decrypted = rsa_decrypt(private_key, &encrypted);
        println!("Original: {} \nEncrypted: {}", data, decrypted);
    }

    {
        let openssl_encrypted = openssl_rsa_encrypt(public_key, data);
        // println!("Encrypted: {}", encrypted);
        let openssl_encrypted = openssl_rsa_decrypt(private_key, &openssl_encrypted);
        println!("Original: {} \nEncrypted: {}", data, openssl_encrypted);
    }

    println!("Hello, world!");
}
