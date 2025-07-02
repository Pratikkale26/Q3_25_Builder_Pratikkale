export type Turbin3Prereq = {
  "version": "0.1.0",
  "name":"Turbin3_prereq",
  "address": string;
  "metadata": {
    address: string;
    name: string;
    version: string;
    spec: string;
  };
  "instructions": [
    {
      "name": "submit_rs",
      "discriminator": [
        77,
        124,
        82,
        163,
        21,
        133,
        181,
        206
      ],
      "accounts": [
        {
          "name": "user",
          "writable": true,
          "signer": true
        },
        {
          "name": "account",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  114,
                  101,
                  114,
                  101,
                  113,
                  115
                ]
              },
              {
                "kind": "account",
                "path": "user"
              }
            ]
          }
        },
        {
          "name": "mint",
          "writable": true,
          "signer": true
        },
        {
          "name": "collection",
          "writable": true
        },
        {
          "name": "authority",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  108,
                  108,
                  101,
                  99,
                  116,
                  105,
                  111,
                  110
                ]
              },
              {
                "kind": "account",
                "path": "collection"
              }
            ]
          }
        },
        {
          "name": "mpl_core_program",
          "address": "CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d"
        },
        {
          "name": "system_program",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
  ]}

export const IDL: Turbin3Prereq = {
  "version": "0.1.0",
  "name": "Turbin3_prereq",
  "address": "TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM",
  "metadata": {
    address: "TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM",
    name: "Turbin3_prereq",
    version: "0.1.0",
    spec: "0.1.0"
  },
  "instructions": [
    {
      "name": "submit_rs",
      "discriminator": [
        77,
        124,
        82,
        163,
        21,
        133,
        181,
        206
      ],
      "accounts": [
        {
          "name": "user",
          "writable": true,
          "signer": true
        },
        {
          "name": "account",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  114,
                  101,
                  114,
                  101,
                  113,
                  115
                ]
              },
              {
                "kind": "account",
                "path": "user"
              }
            ]
          }
        },
        {
          "name": "mint",
          "writable": true,
          "signer": true
        },
        {
          "name": "collection",
          "writable": true
        },
        {
          "name": "authority",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  108,
                  108,
                  101,
                  99,
                  116,
                  105,
                  111,
                  110
                ]
              },
              {
                "kind": "account",
                "path": "collection"
              }
            ]
          }
        },
        {
          "name": "mpl_core_program",
          "address": "CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d"
        },
        {
          "name": "system_program",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
  ] }