#!/bin/bash

PACKAGES="
  cw20
  cw3
"
CONTRACTS="cw1-whitelist cw1-subkeys cw20-base cw20-ics20 cw4-stake cw4-group cw3-fixed-multisig cw3-flex-multisig"

# for pack in $PACKAGES; do
#   (
#     cd "packages/$pack"
#     echo "Publishing $pack"
#     cargo publish
#   )
# done

# for lib in $CONTRACTS; do
#   (
#     cd "contracts/$lib"
#     echo "Publishing $lib"
#     cargo publish
#   )
# done


(
  cd "packages/interface"
  echo "Publishing interface"
  cargo publish
)
