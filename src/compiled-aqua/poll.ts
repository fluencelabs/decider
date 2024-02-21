/* eslint-disable */
// @ts-nocheck
/**
 *
 * This file is generated using:
 * @fluencelabs/aqua-api version: 0.13.0
 * @fluencelabs/aqua-to-js version: 0.3.5
 * If you find any bugs in generated AIR, please write an issue on GitHub: https://github.com/fluencelabs/aqua/issues
 * If you find any bugs in generated JS/TS, please write an issue on GitHub: https://github.com/fluencelabs/js-client/issues
 *
 */
import type { IFluenceClient as IFluenceClient$$, ParticleContext as ParticleContext$$ } from '@fluencelabs/js-client';

// Making aliases to reduce chance of accidental name collision
import {
    v5_callFunction as callFunction$$,
    v5_registerService as registerService$$
} from '@fluencelabs/js-client';


// Functions
export const main_script = `
(xor
 (seq
  (seq
   (seq
    (seq
     (seq
      (seq
       (call %init_peer_id% ("getDataSrv" "-relay-") [] -relay-)
       (call %init_peer_id% ("getDataSrv" "spell_id") [] -spell_id-arg-)
      )
      (call %init_peer_id% ("getDataSrv" "chain") [] -chain-arg-)
     )
     (call %init_peer_id% ("getDataSrv" "worker_settings") [] -worker_settings-arg-)
    )
    (new $latest
     (seq
      (seq
       (seq
        (call %init_peer_id% ("chain_connector" "latest_block_number") [-chain-arg-.$.api_endpoint] ret)
        (new -if-else-error-
         (new -else-error-
          (new -if-error-
           (xor
            (match ret.$.success true
             (ap ret.$.number_hex $latest)
            )
            (seq
             (ap :error: -if-error-)
             (xor
              (match :error:.$.error_code 10001
               (seq
                (seq
                 (seq
                  (new $array-inline
                   (seq
                    (seq
                     (ap "error retrieving latest block number" $array-inline)
                     (ap ret.$.error $array-inline)
                    )
                    (canon %init_peer_id% $array-inline  #array-inline-0)
                   )
                  )
                  (par
                   (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-0])
                   (null)
                  )
                 )
                 (call %init_peer_id% ("json" "stringify") [#array-inline-0] ret-0)
                )
                (call %init_peer_id% (-spell_id-arg- "store_log") [ret-0] ret-1)
               )
              )
              (seq
               (seq
                (ap :error: -else-error-)
                (xor
                 (match :error:.$.error_code 10001
                  (ap -if-error- -if-else-error-)
                 )
                 (ap -else-error- -if-else-error-)
                )
               )
               (fail -if-else-error-)
              )
             )
            )
           )
          )
         )
        )
       )
       (canon %init_peer_id% $latest  #-latest-fix-0)
      )
      (ap #-latest-fix-0 -latest-flat-0)
     )
    )
   )
   (new -if-error-
    (xor
     (mismatch -latest-flat-0 []
      (seq
       (seq
        (seq
         (seq
          (seq
           (seq
            (seq
             (new $joined_deals-0
              (seq
               (seq
                (seq
                 (seq
                  (call %init_peer_id% (-spell_id-arg- "list_get_strings") ["joined_deals"] ret-2)
                  (xor
                   (match ret-2.$.success true
                    (ap false not)
                   )
                   (ap true not)
                  )
                 )
                 (new -if-else-error-
                  (new -else-error-
                   (new -if-error-
                    (xor
                     (match not true
                      (seq
                       (seq
                        (seq
                         (new $array-inline-1
                          (seq
                           (seq
                            (ap "can't restrive joined deals" $array-inline-1)
                            (ap ret-2.$.error $array-inline-1)
                           )
                           (canon %init_peer_id% $array-inline-1  #array-inline-1-0)
                          )
                         )
                         (par
                          (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-1-0])
                          (null)
                         )
                        )
                        (call %init_peer_id% ("json" "stringify") [#array-inline-1-0] ret-3)
                       )
                       (call %init_peer_id% (-spell_id-arg- "store_log") [ret-3] ret-4)
                      )
                     )
                     (seq
                      (ap :error: -if-error-)
                      (xor
                       (match :error:.$.error_code 10001
                        (fold ret-2.$.value joined_deal_str-0
                         (seq
                          (xor
                           (seq
                            (call %init_peer_id% ("json" "parse") [joined_deal_str-0] ret-5)
                            (ap ret-5 $joined_deals-0)
                           )
                           (seq
                            (seq
                             (seq
                              (new $array-inline-2
                               (seq
                                (seq
                                 (seq
                                  (ap "error parsing JoinedDeal" $array-inline-2)
                                  (ap joined_deal_str-0 $array-inline-2)
                                 )
                                 (ap :error: $array-inline-2)
                                )
                                (canon %init_peer_id% $array-inline-2  #array-inline-2-0)
                               )
                              )
                              (par
                               (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-2-0])
                               (null)
                              )
                             )
                             (call %init_peer_id% ("json" "stringify") [#array-inline-2-0] ret-6)
                            )
                            (call %init_peer_id% (-spell_id-arg- "store_log") [ret-6] ret-7)
                           )
                          )
                          (next joined_deal_str-0)
                         )
                         (null)
                        )
                       )
                       (seq
                        (seq
                         (ap :error: -else-error-)
                         (xor
                          (match :error:.$.error_code 10001
                           (ap -if-error- -if-else-error-)
                          )
                          (ap -else-error- -if-else-error-)
                         )
                        )
                        (fail -if-else-error-)
                       )
                      )
                     )
                    )
                   )
                  )
                 )
                )
                (canon %init_peer_id% $joined_deals-0  #-joined_deals-fix-0)
               )
               (ap #-joined_deals-fix-0 -joined_deals-flat-0)
              )
             )
             (xor
              (seq
               (new $left-0
                (seq
                 (seq
                  (seq
                   (seq
                    (seq
                     (seq
                      (seq
                       (seq
                        (seq
                         (seq
                          (seq
                           (new $result
                            (seq
                             (seq
                              (seq
                               (seq
                                (call %init_peer_id% (-spell_id-arg- "get_string") ["last_seen_block"] ret-8)
                                (xor
                                 (match ret-8.$.success true
                                  (ap false not-0)
                                 )
                                 (ap true not-0)
                                )
                               )
                               (new -if-else-error-
                                (new -else-error-
                                 (new -if-error-
                                  (xor
                                   (match not-0 true
                                    (seq
                                     (seq
                                      (seq
                                       (new $array-inline-3
                                        (seq
                                         (seq
                                          (seq
                                           (ap "get_string failed" $array-inline-3)
                                           (ap "last_seen_block" $array-inline-3)
                                          )
                                          (ap ret-8.$.error $array-inline-3)
                                         )
                                         (canon %init_peer_id% $array-inline-3  #array-inline-3-0)
                                        )
                                       )
                                       (par
                                        (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-3-0])
                                        (null)
                                       )
                                      )
                                      (call %init_peer_id% ("json" "stringify") [#array-inline-3-0] ret-9)
                                     )
                                     (call %init_peer_id% (-spell_id-arg- "store_log") [ret-9] ret-10)
                                    )
                                   )
                                   (seq
                                    (ap :error: -if-error-)
                                    (xor
                                     (match :error:.$.error_code 10001
                                      (seq
                                       (xor
                                        (match ret-8.$.absent true
                                         (ap false not-1)
                                        )
                                        (ap true not-1)
                                       )
                                       (new -if-error-
                                        (xor
                                         (match not-1 true
                                          (ap ret-8.$.value $result)
                                         )
                                         (seq
                                          (ap :error: -if-error-)
                                          (xor
                                           (match :error:.$.error_code 10001
                                            (null)
                                           )
                                           (fail -if-error-)
                                          )
                                         )
                                        )
                                       )
                                      )
                                     )
                                     (seq
                                      (seq
                                       (ap :error: -else-error-)
                                       (xor
                                        (match :error:.$.error_code 10001
                                         (ap -if-error- -if-else-error-)
                                        )
                                        (ap -else-error- -if-else-error-)
                                       )
                                      )
                                      (fail -if-else-error-)
                                     )
                                    )
                                   )
                                  )
                                 )
                                )
                               )
                              )
                              (canon %init_peer_id% $result  #-result-fix-0)
                             )
                             (ap #-result-fix-0 -result-flat-0)
                            )
                           )
                           (new $array-inline-4
                            (seq
                             (seq
                              (ap "maybe_last_seen is" $array-inline-4)
                              (ap -result-flat-0 $array-inline-4)
                             )
                             (canon %init_peer_id% $array-inline-4  #array-inline-4-0)
                            )
                           )
                          )
                          (par
                           (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-4-0])
                           (null)
                          )
                         )
                         (call %init_peer_id% ("json" "stringify") [#array-inline-4-0] ret-11)
                        )
                        (call %init_peer_id% (-spell_id-arg- "store_log") [ret-11] ret-12)
                       )
                       (new -if-else-error-
                        (new -else-error-
                         (new -if-error-
                          (xor
                           (match -result-flat-0 []
                            (seq
                             (seq
                              (seq
                               (seq
                                (seq
                                 (seq
                                  (new $left-1
                                   (seq
                                    (seq
                                     (seq
                                      (new $from_block-1
                                       (seq
                                        (seq
                                         (new $result-0
                                          (seq
                                           (seq
                                            (seq
                                             (seq
                                              (call %init_peer_id% (-spell_id-arg- "get_string") ["from_block"] ret-13)
                                              (xor
                                               (match ret-13.$.success true
                                                (ap false not-2)
                                               )
                                               (ap true not-2)
                                              )
                                             )
                                             (new -if-else-error-
                                              (new -else-error-
                                               (new -if-error-
                                                (xor
                                                 (match not-2 true
                                                  (seq
                                                   (seq
                                                    (seq
                                                     (new $array-inline-5
                                                      (seq
                                                       (seq
                                                        (seq
                                                         (ap "get_string failed" $array-inline-5)
                                                         (ap "from_block" $array-inline-5)
                                                        )
                                                        (ap ret-13.$.error $array-inline-5)
                                                       )
                                                       (canon %init_peer_id% $array-inline-5  #array-inline-5-0)
                                                      )
                                                     )
                                                     (par
                                                      (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-5-0])
                                                      (null)
                                                     )
                                                    )
                                                    (call %init_peer_id% ("json" "stringify") [#array-inline-5-0] ret-14)
                                                   )
                                                   (call %init_peer_id% (-spell_id-arg- "store_log") [ret-14] ret-15)
                                                  )
                                                 )
                                                 (seq
                                                  (ap :error: -if-error-)
                                                  (xor
                                                   (match :error:.$.error_code 10001
                                                    (seq
                                                     (xor
                                                      (match ret-13.$.absent true
                                                       (ap false not-3)
                                                      )
                                                      (ap true not-3)
                                                     )
                                                     (new -if-error-
                                                      (xor
                                                       (match not-3 true
                                                        (ap ret-13.$.value $result-0)
                                                       )
                                                       (seq
                                                        (ap :error: -if-error-)
                                                        (xor
                                                         (match :error:.$.error_code 10001
                                                          (null)
                                                         )
                                                         (fail -if-error-)
                                                        )
                                                       )
                                                      )
                                                     )
                                                    )
                                                   )
                                                   (seq
                                                    (seq
                                                     (ap :error: -else-error-)
                                                     (xor
                                                      (match :error:.$.error_code 10001
                                                       (ap -if-error- -if-else-error-)
                                                      )
                                                      (ap -else-error- -if-else-error-)
                                                     )
                                                    )
                                                    (fail -if-else-error-)
                                                   )
                                                  )
                                                 )
                                                )
                                               )
                                              )
                                             )
                                            )
                                            (canon %init_peer_id% $result-0  #-result-fix-0-0)
                                           )
                                           (ap #-result-fix-0-0 -result-flat-0-0)
                                          )
                                         )
                                         (new -if-else-error-
                                          (new -else-error-
                                           (new -if-error-
                                            (xor
                                             (match -result-flat-0-0 []
                                              (ap "latest" $from_block-1)
                                             )
                                             (seq
                                              (ap :error: -if-error-)
                                              (xor
                                               (match :error:.$.error_code 10001
                                                (seq
                                                 (call %init_peer_id% ("json" "parse") [-result-flat-0-0.$.[0]] ret-16)
                                                 (ap ret-16 $from_block-1)
                                                )
                                               )
                                               (seq
                                                (seq
                                                 (ap :error: -else-error-)
                                                 (xor
                                                  (match :error:.$.error_code 10001
                                                   (ap -if-error- -if-else-error-)
                                                  )
                                                  (ap -else-error- -if-else-error-)
                                                 )
                                                )
                                                (fail -if-else-error-)
                                               )
                                              )
                                             )
                                            )
                                           )
                                          )
                                         )
                                        )
                                        (new $from_block-1_test
                                         (seq
                                          (seq
                                           (fold $from_block-1 from_block-1_fold_var
                                            (seq
                                             (seq
                                              (ap from_block-1_fold_var $from_block-1_test)
                                              (canon %init_peer_id% $from_block-1_test  #from_block-1_iter_canon)
                                             )
                                             (xor
                                              (match #from_block-1_iter_canon.length 1
                                               (null)
                                              )
                                              (next from_block-1_fold_var)
                                             )
                                            )
                                            (never)
                                           )
                                           (canon %init_peer_id% $from_block-1_test  #from_block-1_result_canon)
                                          )
                                          (ap #from_block-1_result_canon from_block-1_gate)
                                         )
                                        )
                                       )
                                      )
                                      (new -if-else-error-
                                       (new -else-error-
                                        (new -if-error-
                                         (xor
                                          (match from_block-1_gate.$.[0] "latest"
                                           (seq
                                            (new $latest-1
                                             (seq
                                              (seq
                                               (seq
                                                (call %init_peer_id% ("chain_connector" "latest_block_number") [-chain-arg-.$.api_endpoint] ret-17)
                                                (new -if-else-error-
                                                 (new -else-error-
                                                  (new -if-error-
                                                   (xor
                                                    (match ret-17.$.success true
                                                     (ap ret-17.$.number_hex $latest-1)
                                                    )
                                                    (seq
                                                     (ap :error: -if-error-)
                                                     (xor
                                                      (match :error:.$.error_code 10001
                                                       (seq
                                                        (seq
                                                         (seq
                                                          (new $array-inline-6
                                                           (seq
                                                            (seq
                                                             (ap "error retrieving latest block number" $array-inline-6)
                                                             (ap ret-17.$.error $array-inline-6)
                                                            )
                                                            (canon %init_peer_id% $array-inline-6  #array-inline-6-0)
                                                           )
                                                          )
                                                          (par
                                                           (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-6-0])
                                                           (null)
                                                          )
                                                         )
                                                         (call %init_peer_id% ("json" "stringify") [#array-inline-6-0] ret-18)
                                                        )
                                                        (call %init_peer_id% (-spell_id-arg- "store_log") [ret-18] ret-19)
                                                       )
                                                      )
                                                      (seq
                                                       (seq
                                                        (ap :error: -else-error-)
                                                        (xor
                                                         (match :error:.$.error_code 10001
                                                          (ap -if-error- -if-else-error-)
                                                         )
                                                         (ap -else-error- -if-else-error-)
                                                        )
                                                       )
                                                       (fail -if-else-error-)
                                                      )
                                                     )
                                                    )
                                                   )
                                                  )
                                                 )
                                                )
                                               )
                                               (canon %init_peer_id% $latest-1  #-latest-fix-0-0)
                                              )
                                              (ap #-latest-fix-0-0 -latest-flat-0-0)
                                             )
                                            )
                                            (new -if-error-
                                             (xor
                                              (mismatch -latest-flat-0-0 []
                                               (ap -latest-flat-0-0.$.[0] $left-1)
                                              )
                                              (seq
                                               (ap :error: -if-error-)
                                               (xor
                                                (match :error:.$.error_code 10002
                                                 (null)
                                                )
                                                (fail -if-error-)
                                               )
                                              )
                                             )
                                            )
                                           )
                                          )
                                          (seq
                                           (ap :error: -if-error-)
                                           (xor
                                            (match :error:.$.error_code 10001
                                             (new -if-else-error-
                                              (new -else-error-
                                               (new -if-error-
                                                (xor
                                                 (match from_block-1_gate.$.[0] "earliest"
                                                  (ap "0x0" $left-1)
                                                 )
                                                 (seq
                                                  (ap :error: -if-error-)
                                                  (xor
                                                   (match :error:.$.error_code 10001
                                                    (ap from_block-1_gate.$.[0] $left-1)
                                                   )
                                                   (seq
                                                    (seq
                                                     (ap :error: -else-error-)
                                                     (xor
                                                      (match :error:.$.error_code 10001
                                                       (ap -if-error- -if-else-error-)
                                                      )
                                                      (ap -else-error- -if-else-error-)
                                                     )
                                                    )
                                                    (fail -if-else-error-)
                                                   )
                                                  )
                                                 )
                                                )
                                               )
                                              )
                                             )
                                            )
                                            (seq
                                             (seq
                                              (ap :error: -else-error-)
                                              (xor
                                               (match :error:.$.error_code 10001
                                                (ap -if-error- -if-else-error-)
                                               )
                                               (ap -else-error- -if-else-error-)
                                              )
                                             )
                                             (fail -if-else-error-)
                                            )
                                           )
                                          )
                                         )
                                        )
                                       )
                                      )
                                     )
                                     (canon %init_peer_id% $left-1  #-left-fix-0-0)
                                    )
                                    (ap #-left-fix-0-0 -left-flat-0-0)
                                   )
                                  )
                                  (new $array-inline-7
                                   (seq
                                    (seq
                                     (ap "init: will set last seen to" $array-inline-7)
                                     (ap -left-flat-0-0 $array-inline-7)
                                    )
                                    (canon %init_peer_id% $array-inline-7  #array-inline-7-0)
                                   )
                                  )
                                 )
                                 (par
                                  (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-7-0])
                                  (null)
                                 )
                                )
                                (call %init_peer_id% ("json" "stringify") [#array-inline-7-0] ret-20)
                               )
                               (call %init_peer_id% (-spell_id-arg- "store_log") [ret-20] ret-21)
                              )
                              (new -if-error-
                               (xor
                                (mismatch -left-flat-0-0 []
                                 (seq
                                  (seq
                                   (seq
                                    (seq
                                     (new $array-inline-8
                                      (seq
                                       (seq
                                        (ap "will set last seen to" $array-inline-8)
                                        (ap -left-flat-0-0.$.[0] $array-inline-8)
                                       )
                                       (canon %init_peer_id% $array-inline-8  #array-inline-8-0)
                                      )
                                     )
                                     (par
                                      (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-8-0])
                                      (null)
                                     )
                                    )
                                    (call %init_peer_id% ("json" "stringify") [#array-inline-8-0] ret-22)
                                   )
                                   (call %init_peer_id% (-spell_id-arg- "store_log") [ret-22] ret-23)
                                  )
                                  (xor
                                   (seq
                                    (call %init_peer_id% (-spell_id-arg- "set_string") ["last_seen_block" -left-flat-0-0.$.[0]] ret-24)
                                    (new -if-else-error-
                                     (new -else-error-
                                      (new -if-error-
                                       (xor
                                        (match ret-24.$.success true
                                         (seq
                                          (seq
                                           (seq
                                            (new $array-inline-9
                                             (seq
                                              (seq
                                               (ap "saved last seen" $array-inline-9)
                                               (ap -left-flat-0-0.$.[0] $array-inline-9)
                                              )
                                              (canon %init_peer_id% $array-inline-9  #array-inline-9-0)
                                             )
                                            )
                                            (par
                                             (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-9-0])
                                             (null)
                                            )
                                           )
                                           (call %init_peer_id% ("json" "stringify") [#array-inline-9-0] ret-25)
                                          )
                                          (call %init_peer_id% (-spell_id-arg- "store_log") [ret-25] ret-26)
                                         )
                                        )
                                        (seq
                                         (ap :error: -if-error-)
                                         (xor
                                          (match :error:.$.error_code 10001
                                           (seq
                                            (seq
                                             (seq
                                              (new $array-inline-10
                                               (seq
                                                (seq
                                                 (ap "error saving last_seen_block" $array-inline-10)
                                                 (ap ret-24.$.error $array-inline-10)
                                                )
                                                (canon %init_peer_id% $array-inline-10  #array-inline-10-0)
                                               )
                                              )
                                              (par
                                               (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-10-0])
                                               (null)
                                              )
                                             )
                                             (call %init_peer_id% ("json" "stringify") [#array-inline-10-0] ret-27)
                                            )
                                            (call %init_peer_id% (-spell_id-arg- "store_log") [ret-27] ret-28)
                                           )
                                          )
                                          (seq
                                           (seq
                                            (ap :error: -else-error-)
                                            (xor
                                             (match :error:.$.error_code 10001
                                              (ap -if-error- -if-else-error-)
                                             )
                                             (ap -else-error- -if-else-error-)
                                            )
                                           )
                                           (fail -if-else-error-)
                                          )
                                         )
                                        )
                                       )
                                      )
                                     )
                                    )
                                   )
                                   (seq
                                    (seq
                                     (seq
                                      (new $array-inline-11
                                       (seq
                                        (seq
                                         (ap "error saving last_seen_block" $array-inline-11)
                                         (ap :error: $array-inline-11)
                                        )
                                        (canon %init_peer_id% $array-inline-11  #array-inline-11-0)
                                       )
                                      )
                                      (par
                                       (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-11-0])
                                       (null)
                                      )
                                     )
                                     (call %init_peer_id% ("json" "stringify") [#array-inline-11-0] ret-29)
                                    )
                                    (call %init_peer_id% (-spell_id-arg- "store_log") [ret-29] ret-30)
                                   )
                                  )
                                 )
                                )
                                (seq
                                 (ap :error: -if-error-)
                                 (xor
                                  (match :error:.$.error_code 10002
                                   (null)
                                  )
                                  (fail -if-error-)
                                 )
                                )
                               )
                              )
                             )
                             (new -if-error-
                              (xor
                               (mismatch -left-flat-0-0 []
                                (ap -left-flat-0-0.$.[0] $left-0)
                               )
                               (seq
                                (ap :error: -if-error-)
                                (xor
                                 (match :error:.$.error_code 10002
                                  (null)
                                 )
                                 (fail -if-error-)
                                )
                               )
                              )
                             )
                            )
                           )
                           (seq
                            (ap :error: -if-error-)
                            (xor
                             (match :error:.$.error_code 10001
                              (seq
                               (seq
                                (seq
                                 (seq
                                  (call %init_peer_id% ("chain_connector" "hex_add") [-result-flat-0.$.[0] 1] ret-31)
                                  (xor
                                   (match ret-31.$.success true
                                    (ap false not-4)
                                   )
                                   (ap true not-4)
                                  )
                                 )
                                 (new -if-error-
                                  (xor
                                   (match not-4 true
                                    (seq
                                     (seq
                                      (seq
                                       (new $array-inline-12
                                        (seq
                                         (seq
                                          (seq
                                           (ap "hex_add failed" $array-inline-12)
                                           (ap -result-flat-0.$.[0] $array-inline-12)
                                          )
                                          (ap 1 $array-inline-12)
                                         )
                                         (canon %init_peer_id% $array-inline-12  #array-inline-12-0)
                                        )
                                       )
                                       (par
                                        (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-12-0])
                                        (null)
                                       )
                                      )
                                      (call %init_peer_id% ("json" "stringify") [#array-inline-12-0] ret-32)
                                     )
                                     (call %init_peer_id% (-spell_id-arg- "store_log") [ret-32] ret-33)
                                    )
                                   )
                                   (seq
                                    (ap :error: -if-error-)
                                    (xor
                                     (match :error:.$.error_code 10001
                                      (null)
                                     )
                                     (fail -if-error-)
                                    )
                                   )
                                  )
                                 )
                                )
                                (ap ret-31.$.hex ret-31_flat)
                               )
                               (new -if-else-error-
                                (new -else-error-
                                 (new -if-error-
                                  (xor
                                   (mismatch ret-31_flat []
                                    (ap ret-31_flat.$.[0] $left-0)
                                   )
                                   (seq
                                    (ap :error: -if-error-)
                                    (xor
                                     (match :error:.$.error_code 10002
                                      (ap -result-flat-0.$.[0] $left-0)
                                     )
                                     (seq
                                      (seq
                                       (ap :error: -else-error-)
                                       (xor
                                        (match :error:.$.error_code 10001
                                         (ap -if-error- -if-else-error-)
                                        )
                                        (ap -else-error- -if-else-error-)
                                       )
                                      )
                                      (fail -if-else-error-)
                                     )
                                    )
                                   )
                                  )
                                 )
                                )
                               )
                              )
                             )
                             (seq
                              (seq
                               (ap :error: -else-error-)
                               (xor
                                (match :error:.$.error_code 10001
                                 (ap -if-error- -if-else-error-)
                                )
                                (ap -else-error- -if-else-error-)
                               )
                              )
                              (fail -if-else-error-)
                             )
                            )
                           )
                          )
                         )
                        )
                       )
                      )
                      (new $array-inline-13
                       (seq
                        (seq
                         (seq
                          (ap "left boundary will be" $array-inline-13)
                          (canon %init_peer_id% $left-0  #push-to-stream-522)
                         )
                         (ap #push-to-stream-522 $array-inline-13)
                        )
                        (canon %init_peer_id% $array-inline-13  #array-inline-13-0)
                       )
                      )
                     )
                     (par
                      (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-13-0])
                      (null)
                     )
                    )
                    (call %init_peer_id% ("json" "stringify") [#array-inline-13-0] ret-34)
                   )
                   (call %init_peer_id% (-spell_id-arg- "store_log") [ret-34] ret-35)
                  )
                  (canon %init_peer_id% $left-0  #-left-fix-0)
                 )
                 (ap #-left-fix-0 -left-flat-0)
                )
               )
               (new -if-else-error-
                (new -else-error-
                 (new -if-error-
                  (xor
                   (mismatch -left-flat-0 []
                    (seq
                     (new $poll-0
                      (seq
                       (seq
                        (seq
                         (seq
                          (call %init_peer_id% ("chain_connector" "poll_deal_matches") [-chain-arg- -left-flat-0.$.[0]] ret-36)
                          (xor
                           (match ret-36.$.success true
                            (ap false not-5)
                           )
                           (ap true not-5)
                          )
                         )
                         (new -if-else-error-
                          (new -else-error-
                           (new -if-error-
                            (xor
                             (match not-5 true
                              (seq
                               (seq
                                (seq
                                 (new $array-inline-14
                                  (seq
                                   (seq
                                    (ap "error polling deal created events" $array-inline-14)
                                    (ap ret-36.$.error $array-inline-14)
                                   )
                                   (canon %init_peer_id% $array-inline-14  #array-inline-14-0)
                                  )
                                 )
                                 (par
                                  (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-14-0])
                                  (null)
                                 )
                                )
                                (call %init_peer_id% ("json" "stringify") [#array-inline-14-0] ret-37)
                               )
                               (call %init_peer_id% (-spell_id-arg- "store_log") [ret-37] ret-38)
                              )
                             )
                             (seq
                              (ap :error: -if-error-)
                              (xor
                               (match :error:.$.error_code 10001
                                (seq
                                 (seq
                                  (seq
                                   (seq
                                    (seq
                                     (seq
                                      (ap ret-36.$.logs ret-36_flat)
                                      (new $array-inline-15
                                       (seq
                                        (seq
                                         (seq
                                          (seq
                                           (seq
                                            (seq
                                             (ap ret-36_flat ret-36_flat_to_functor)
                                             (ap ret-36_flat_to_functor.length ret-36_flat_length)
                                            )
                                            (ap "new deals from poll:" $array-inline-15)
                                           )
                                           (ap ret-36_flat_length $array-inline-15)
                                          )
                                          (ap "from block:" $array-inline-15)
                                         )
                                         (ap -left-flat-0.$.[0] $array-inline-15)
                                        )
                                        (canon %init_peer_id% $array-inline-15  #array-inline-15-0)
                                       )
                                      )
                                     )
                                     (par
                                      (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-15-0])
                                      (null)
                                     )
                                    )
                                    (call %init_peer_id% ("json" "stringify") [#array-inline-15-0] ret-39)
                                   )
                                   (call %init_peer_id% (-spell_id-arg- "store_log") [ret-39] ret-40)
                                  )
                                  (new %Poll_obj_map
                                   (seq
                                    (seq
                                     (ap ("logs" ret-36_flat) %Poll_obj_map)
                                     (ap ("right_boundary" ret-36.$.right_boundary) %Poll_obj_map)
                                    )
                                    (canon %init_peer_id% %Poll_obj_map  Poll_obj)
                                   )
                                  )
                                 )
                                 (ap Poll_obj $poll-0)
                                )
                               )
                               (seq
                                (seq
                                 (ap :error: -else-error-)
                                 (xor
                                  (match :error:.$.error_code 10001
                                   (ap -if-error- -if-else-error-)
                                  )
                                  (ap -else-error- -if-else-error-)
                                 )
                                )
                                (fail -if-else-error-)
                               )
                              )
                             )
                            )
                           )
                          )
                         )
                        )
                        (canon %init_peer_id% $poll-0  #-poll-fix-0)
                       )
                       (ap #-poll-fix-0 -poll-flat-0)
                      )
                     )
                     (new -if-error-
                      (xor
                       (mismatch -poll-flat-0 []
                        (seq
                         (seq
                          (seq
                           (seq
                            (seq
                             (seq
                              (seq
                               (seq
                                (seq
                                 (seq
                                  (new $new
                                   (seq
                                    (seq
                                     (fold -poll-flat-0.$.[0].logs match-0
                                      (seq
                                       (new $joined
                                        (seq
                                         (seq
                                          (seq
                                           (seq
                                            (seq
                                             (ap match-0.$.info match-0_flat)
                                             (ap match-0_flat.$.deal_id match-0_flat_flat)
                                            )
                                            (new $created
                                             (seq
                                              (seq
                                               (call %init_peer_id% ("worker" "get_worker_id") [match-0_flat_flat] ret-41)
                                               (new -if-else-error-
                                                (new -else-error-
                                                 (new -if-error-
                                                  (xor
                                                   (mismatch ret-41 []
                                                    (ap true $created)
                                                   )
                                                   (seq
                                                    (ap :error: -if-error-)
                                                    (xor
                                                     (match :error:.$.error_code 10002
                                                      (ap false $created)
                                                     )
                                                     (seq
                                                      (seq
                                                       (ap :error: -else-error-)
                                                       (xor
                                                        (match :error:.$.error_code 10001
                                                         (ap -if-error- -if-else-error-)
                                                        )
                                                        (ap -else-error- -if-else-error-)
                                                       )
                                                      )
                                                      (fail -if-else-error-)
                                                     )
                                                    )
                                                   )
                                                  )
                                                 )
                                                )
                                               )
                                              )
                                              (new $created_test
                                               (seq
                                                (seq
                                                 (fold $created created_fold_var
                                                  (seq
                                                   (seq
                                                    (ap created_fold_var $created_test)
                                                    (canon %init_peer_id% $created_test  #created_iter_canon)
                                                   )
                                                   (xor
                                                    (match #created_iter_canon.length 1
                                                     (null)
                                                    )
                                                    (next created_fold_var)
                                                   )
                                                  )
                                                  (never)
                                                 )
                                                 (canon %init_peer_id% $created_test  #created_result_canon)
                                                )
                                                (ap #created_result_canon created_gate)
                                               )
                                              )
                                             )
                                            )
                                           )
                                           (new -if-error-
                                            (xor
                                             (match created_gate.$.[0] true
                                              (fold -joined_deals-flat-0 deal-0
                                               (seq
                                                (new -if-error-
                                                 (xor
                                                  (match match-0_flat_flat deal-0.$.deal_id
                                                   (ap true $joined)
                                                  )
                                                  (seq
                                                   (ap :error: -if-error-)
                                                   (xor
                                                    (match :error:.$.error_code 10001
                                                     (null)
                                                    )
                                                    (fail -if-error-)
                                                   )
                                                  )
                                                 )
                                                )
                                                (next deal-0)
                                               )
                                               (null)
                                              )
                                             )
                                             (seq
                                              (ap :error: -if-error-)
                                              (xor
                                               (match :error:.$.error_code 10001
                                                (null)
                                               )
                                               (fail -if-error-)
                                              )
                                             )
                                            )
                                           )
                                          )
                                          (canon %init_peer_id% $joined  #joined_canon)
                                         )
                                         (new -if-else-error-
                                          (new -else-error-
                                           (new -if-error-
                                            (xor
                                             (match #joined_canon []
                                              (ap match-0 $new)
                                             )
                                             (seq
                                              (ap :error: -if-error-)
                                              (xor
                                               (match :error:.$.error_code 10001
                                                (seq
                                                 (seq
                                                  (seq
                                                   (seq
                                                    (call %init_peer_id% ("op" "concat_strings") ["decider deal_id=" match-0_flat_flat ": "] ret-42)
                                                    (par
                                                     (call %init_peer_id% ("run-console" "print") [ret-42 "deal is already joined"])
                                                     (null)
                                                    )
                                                   )
                                                   (call %init_peer_id% ("json" "stringify") ["deal is already joined"] ret-43)
                                                  )
                                                  (call %init_peer_id% ("op" "concat_strings") [ret-42 ret-43] ret-44)
                                                 )
                                                 (call %init_peer_id% (-spell_id-arg- "store_log") [ret-44] ret-45)
                                                )
                                               )
                                               (seq
                                                (seq
                                                 (ap :error: -else-error-)
                                                 (xor
                                                  (match :error:.$.error_code 10001
                                                   (ap -if-error- -if-else-error-)
                                                  )
                                                  (ap -else-error- -if-else-error-)
                                                 )
                                                )
                                                (fail -if-else-error-)
                                               )
                                              )
                                             )
                                            )
                                           )
                                          )
                                         )
                                        )
                                       )
                                       (next match-0)
                                      )
                                      (null)
                                     )
                                     (canon %init_peer_id% $new  #-new-fix-0)
                                    )
                                    (ap #-new-fix-0 -new-flat-0)
                                   )
                                  )
                                  (new $deal_ids
                                   (seq
                                    (seq
                                     (fold -new-flat-0 log-16-0
                                      (seq
                                       (seq
                                        (seq
                                         (seq
                                          (seq
                                           (ap log-16-0.$.info log-16-0_flat)
                                           (ap log-16-0_flat.$.deal_id log-16-0_flat_flat)
                                          )
                                          (new $array-inline-16
                                           (seq
                                            (ap log-16-0.$.info.unit_id $array-inline-16)
                                            (canon %init_peer_id% $array-inline-16  #array-inline-16-0)
                                           )
                                          )
                                         )
                                         (new $error-0
                                          (new $worker_id-0
                                           (seq
                                            (seq
                                             (seq
                                              (seq
                                               (seq
                                                (seq
                                                 (seq
                                                  (seq
                                                   (seq
                                                    (seq
                                                     (seq
                                                      (seq
                                                       (new $array-inline-17
                                                        (seq
                                                         (seq
                                                          (ap "joining a deal from_block" $array-inline-17)
                                                          (ap log-16-0.$.block_number $array-inline-17)
                                                         )
                                                         (canon %init_peer_id% $array-inline-17  #array-inline-17-0)
                                                        )
                                                       )
                                                       (call %init_peer_id% ("op" "concat_strings") ["decider deal_id=" log-16-0_flat_flat ": "] ret-46)
                                                      )
                                                      (par
                                                       (call %init_peer_id% ("run-console" "print") [ret-46 #array-inline-17-0])
                                                       (null)
                                                      )
                                                     )
                                                     (call %init_peer_id% ("json" "stringify") [#array-inline-17-0] ret-47)
                                                    )
                                                    (call %init_peer_id% ("op" "concat_strings") [ret-46 ret-47] ret-48)
                                                   )
                                                   (call %init_peer_id% (-spell_id-arg- "store_log") [ret-48] ret-49)
                                                  )
                                                  (xor
                                                   (seq
                                                    (new $worker_id-1
                                                     (seq
                                                      (seq
                                                       (seq
                                                        (call %init_peer_id% ("worker" "get_worker_id") [log-16-0_flat_flat] ret-50)
                                                        (new -if-else-error-
                                                         (new -else-error-
                                                          (new -if-error-
                                                           (xor
                                                            (match ret-50 []
                                                             (xor
                                                              (seq
                                                               (seq
                                                                (seq
                                                                 (seq
                                                                  (seq
                                                                   (seq
                                                                    (seq
                                                                     (call %init_peer_id% ("worker" "create") [log-16-0_flat_flat #array-inline-16-0] ret-51)
                                                                     (ap ret-51 $worker_id-1)
                                                                    )
                                                                    (new $array-inline-18
                                                                     (seq
                                                                      (seq
                                                                       (seq
                                                                        (new $worker_id-1_test
                                                                         (seq
                                                                          (seq
                                                                           (fold $worker_id-1 worker_id-1_fold_var
                                                                            (seq
                                                                             (seq
                                                                              (ap worker_id-1_fold_var $worker_id-1_test)
                                                                              (canon %init_peer_id% $worker_id-1_test  #worker_id-1_iter_canon)
                                                                             )
                                                                             (xor
                                                                              (match #worker_id-1_iter_canon.length 1
                                                                               (null)
                                                                              )
                                                                              (next worker_id-1_fold_var)
                                                                             )
                                                                            )
                                                                            (never)
                                                                           )
                                                                           (canon %init_peer_id% $worker_id-1_test  #worker_id-1_result_canon)
                                                                          )
                                                                          (ap #worker_id-1_result_canon worker_id-1_gate)
                                                                         )
                                                                        )
                                                                        (ap "created worker for deal" $array-inline-18)
                                                                       )
                                                                       (ap worker_id-1_gate.$.[0] $array-inline-18)
                                                                      )
                                                                      (canon %init_peer_id% $array-inline-18  #array-inline-18-0)
                                                                     )
                                                                    )
                                                                   )
                                                                   (call %init_peer_id% ("op" "concat_strings") ["decider deal_id=" log-16-0_flat_flat ": "] ret-52)
                                                                  )
                                                                  (par
                                                                   (call %init_peer_id% ("run-console" "print") [ret-52 #array-inline-18-0])
                                                                   (null)
                                                                  )
                                                                 )
                                                                 (call %init_peer_id% ("json" "stringify") [#array-inline-18-0] ret-53)
                                                                )
                                                                (call %init_peer_id% ("op" "concat_strings") [ret-52 ret-53] ret-54)
                                                               )
                                                               (call %init_peer_id% (-spell_id-arg- "store_log") [ret-54] ret-55)
                                                              )
                                                              (seq
                                                               (seq
                                                                (seq
                                                                 (seq
                                                                  (seq
                                                                   (new $array-inline-19
                                                                    (seq
                                                                     (seq
                                                                      (ap "error creating worker" $array-inline-19)
                                                                      (ap :error:.$.message $array-inline-19)
                                                                     )
                                                                     (canon %init_peer_id% $array-inline-19  #array-inline-19-0)
                                                                    )
                                                                   )
                                                                   (call %init_peer_id% ("op" "concat_strings") ["decider deal_id=" log-16-0_flat_flat ": "] ret-56)
                                                                  )
                                                                  (par
                                                                   (call %init_peer_id% ("run-console" "print") [ret-56 #array-inline-19-0])
                                                                   (null)
                                                                  )
                                                                 )
                                                                 (call %init_peer_id% ("json" "stringify") [#array-inline-19-0] ret-57)
                                                                )
                                                                (call %init_peer_id% ("op" "concat_strings") [ret-56 ret-57] ret-58)
                                                               )
                                                               (call %init_peer_id% (-spell_id-arg- "store_log") [ret-58] ret-59)
                                                              )
                                                             )
                                                            )
                                                            (seq
                                                             (ap :error: -if-error-)
                                                             (xor
                                                              (match :error:.$.error_code 10001
                                                               (seq
                                                                (seq
                                                                 (seq
                                                                  (seq
                                                                   (seq
                                                                    (seq
                                                                     (new $array-inline-20
                                                                      (seq
                                                                       (seq
                                                                        (ap "found existing worker" $array-inline-20)
                                                                        (ap ret-50.$.[0] $array-inline-20)
                                                                       )
                                                                       (canon %init_peer_id% $array-inline-20  #array-inline-20-0)
                                                                      )
                                                                     )
                                                                     (call %init_peer_id% ("op" "concat_strings") ["decider deal_id=" log-16-0_flat_flat ": "] ret-60)
                                                                    )
                                                                    (par
                                                                     (call %init_peer_id% ("run-console" "print") [ret-60 #array-inline-20-0])
                                                                     (null)
                                                                    )
                                                                   )
                                                                   (call %init_peer_id% ("json" "stringify") [#array-inline-20-0] ret-61)
                                                                  )
                                                                  (call %init_peer_id% ("op" "concat_strings") [ret-60 ret-61] ret-62)
                                                                 )
                                                                 (call %init_peer_id% (-spell_id-arg- "store_log") [ret-62] ret-63)
                                                                )
                                                                (ap ret-50.$.[0] $worker_id-1)
                                                               )
                                                              )
                                                              (seq
                                                               (seq
                                                                (ap :error: -else-error-)
                                                                (xor
                                                                 (match :error:.$.error_code 10001
                                                                  (ap -if-error- -if-else-error-)
                                                                 )
                                                                 (ap -else-error- -if-else-error-)
                                                                )
                                                               )
                                                               (fail -if-else-error-)
                                                              )
                                                             )
                                                            )
                                                           )
                                                          )
                                                         )
                                                        )
                                                       )
                                                       (canon %init_peer_id% $worker_id-1  #-worker_id-fix-0-0)
                                                      )
                                                      (ap #-worker_id-fix-0-0 -worker_id-flat-0-0)
                                                     )
                                                    )
                                                    (new -if-error-
                                                     (xor
                                                      (mismatch -worker_id-flat-0-0 []
                                                       (ap -worker_id-flat-0-0.$.[0] $worker_id-0)
                                                      )
                                                      (seq
                                                       (ap :error: -if-error-)
                                                       (xor
                                                        (match :error:.$.error_code 10002
                                                         (null)
                                                        )
                                                        (fail -if-error-)
                                                       )
                                                      )
                                                     )
                                                    )
                                                   )
                                                   (seq
                                                    (seq
                                                     (seq
                                                      (seq
                                                       (seq
                                                        (seq
                                                         (seq
                                                          (seq
                                                           (new $array-inline-21
                                                            (seq
                                                             (seq
                                                              (ap "error creating worker" $array-inline-21)
                                                              (ap :error: $array-inline-21)
                                                             )
                                                             (canon %init_peer_id% $array-inline-21  #array-inline-21-0)
                                                            )
                                                           )
                                                           (call %init_peer_id% ("json" "stringify") [#array-inline-21-0] ret-64)
                                                          )
                                                          (ap ret-64 $error-0)
                                                         )
                                                         (new $array-inline-22
                                                          (seq
                                                           (seq
                                                            (ap "error creating worker" $array-inline-22)
                                                            (ap :error: $array-inline-22)
                                                           )
                                                           (canon %init_peer_id% $array-inline-22  #array-inline-22-0)
                                                          )
                                                         )
                                                        )
                                                        (call %init_peer_id% ("op" "concat_strings") ["decider deal_id=" log-16-0_flat_flat ": "] ret-65)
                                                       )
                                                       (par
                                                        (call %init_peer_id% ("run-console" "print") [ret-65 #array-inline-22-0])
                                                        (null)
                                                       )
                                                      )
                                                      (call %init_peer_id% ("json" "stringify") [#array-inline-22-0] ret-66)
                                                     )
                                                     (call %init_peer_id% ("op" "concat_strings") [ret-65 ret-66] ret-67)
                                                    )
                                                    (call %init_peer_id% (-spell_id-arg- "store_log") [ret-67] ret-68)
                                                   )
                                                  )
                                                 )
                                                 (canon %init_peer_id% $worker_id-0  #worker_id-0_canon)
                                                )
                                                (new -if-error-
                                                 (xor
                                                  (mismatch #worker_id-0_canon []
                                                   (seq
                                                    (call %init_peer_id% ("worker" "is_active") [log-16-0_flat_flat] ret-69)
                                                    (new -if-error-
                                                     (xor
                                                      (match ret-69 true
                                                       (seq
                                                        (seq
                                                         (new $worker_id-0_test
                                                          (seq
                                                           (seq
                                                            (fold $worker_id-0 worker_id-0_fold_var
                                                             (seq
                                                              (seq
                                                               (ap worker_id-0_fold_var $worker_id-0_test)
                                                               (canon %init_peer_id% $worker_id-0_test  #worker_id-0_iter_canon)
                                                              )
                                                              (xor
                                                               (match #worker_id-0_iter_canon.length 1
                                                                (null)
                                                               )
                                                               (next worker_id-0_fold_var)
                                                              )
                                                             )
                                                             (never)
                                                            )
                                                            (canon %init_peer_id% $worker_id-0_test  #worker_id-0_result_canon)
                                                           )
                                                           (ap #worker_id-0_result_canon worker_id-0_gate)
                                                          )
                                                         )
                                                         (xor
                                                          (seq
                                                           (new $-ephemeral-stream-
                                                            (new #-ephemeral-canon-
                                                             (canon -relay- $-ephemeral-stream-  #-ephemeral-canon-)
                                                            )
                                                           )
                                                           (xor
                                                            (xor
                                                             (xor
                                                              (seq
                                                               (seq
                                                                (seq
                                                                 (seq
                                                                  (seq
                                                                   (seq
                                                                    (seq
                                                                     (call worker_id-0_gate.$.[0] ("srv" "resolve_alias") ["worker-spell"] ret-70)
                                                                     (new $array-inline-23
                                                                      (seq
                                                                       (seq
                                                                        (ap "resolved existing worker-spell" $array-inline-23)
                                                                        (ap ret-70 $array-inline-23)
                                                                       )
                                                                       (canon worker_id-0_gate.$.[0] $array-inline-23  #array-inline-23-0)
                                                                      )
                                                                     )
                                                                    )
                                                                    (new $-ephemeral-stream-
                                                                     (new #-ephemeral-canon-
                                                                      (canon -relay- $-ephemeral-stream-  #-ephemeral-canon-)
                                                                     )
                                                                    )
                                                                   )
                                                                   (call %init_peer_id% ("op" "concat_strings") ["decider deal_id=" log-16-0_flat_flat ": "] ret-71)
                                                                  )
                                                                  (par
                                                                   (call %init_peer_id% ("run-console" "print") [ret-71 #array-inline-23-0])
                                                                   (null)
                                                                  )
                                                                 )
                                                                 (call %init_peer_id% ("json" "stringify") [#array-inline-23-0] ret-72)
                                                                )
                                                                (call %init_peer_id% ("op" "concat_strings") [ret-71 ret-72] ret-73)
                                                               )
                                                               (call %init_peer_id% (-spell_id-arg- "store_log") [ret-73] ret-74)
                                                              )
                                                              (seq
                                                               (seq
                                                                (seq
                                                                 (seq
                                                                  (seq
                                                                   (seq
                                                                    (seq
                                                                     (seq
                                                                      (seq
                                                                       (seq
                                                                        (new %WorkerArgs_obj_map
                                                                         (seq
                                                                          (seq
                                                                           (seq
                                                                            (ap ("deal_id" log-16-0_flat_flat) %WorkerArgs_obj_map)
                                                                            (ap ("ipfs" -worker_settings-arg-.$.ipfs) %WorkerArgs_obj_map)
                                                                           )
                                                                           (ap ("worker_def_cid" log-16-0.$.info.app_cid) %WorkerArgs_obj_map)
                                                                          )
                                                                          (canon worker_id-0_gate.$.[0] %WorkerArgs_obj_map  WorkerArgs_obj)
                                                                         )
                                                                        )
                                                                        (par
                                                                         (par
                                                                          (new %BlockChainConfig_obj_map
                                                                           (seq
                                                                            (seq
                                                                             (ap ("end_block" 0) %BlockChainConfig_obj_map)
                                                                             (ap ("start_block" 0) %BlockChainConfig_obj_map)
                                                                            )
                                                                            (canon worker_id-0_gate.$.[0] %BlockChainConfig_obj_map  BlockChainConfig_obj)
                                                                           )
                                                                          )
                                                                          (new %ClockConfig_obj_map
                                                                           (seq
                                                                            (seq
                                                                             (seq
                                                                              (ap ("end_sec" 0) %ClockConfig_obj_map)
                                                                              (ap ("period_sec" 0) %ClockConfig_obj_map)
                                                                             )
                                                                             (ap ("start_sec" 0) %ClockConfig_obj_map)
                                                                            )
                                                                            (canon worker_id-0_gate.$.[0] %ClockConfig_obj_map  ClockConfig_obj)
                                                                           )
                                                                          )
                                                                         )
                                                                         (new %ConnectionPoolConfig_obj_map
                                                                          (seq
                                                                           (seq
                                                                            (ap ("connect" false) %ConnectionPoolConfig_obj_map)
                                                                            (ap ("disconnect" false) %ConnectionPoolConfig_obj_map)
                                                                           )
                                                                           (canon worker_id-0_gate.$.[0] %ConnectionPoolConfig_obj_map  ConnectionPoolConfig_obj)
                                                                          )
                                                                         )
                                                                        )
                                                                       )
                                                                       (new %TriggerConfig_obj_map
                                                                        (seq
                                                                         (seq
                                                                          (seq
                                                                           (ap ("blockchain" BlockChainConfig_obj) %TriggerConfig_obj_map)
                                                                           (ap ("clock" ClockConfig_obj) %TriggerConfig_obj_map)
                                                                          )
                                                                          (ap ("connections" ConnectionPoolConfig_obj) %TriggerConfig_obj_map)
                                                                         )
                                                                         (canon worker_id-0_gate.$.[0] %TriggerConfig_obj_map  TriggerConfig_obj)
                                                                        )
                                                                       )
                                                                      )
                                                                      (call worker_id-0_gate.$.[0] ("spell" "install") [-worker_settings-arg-.$.script WorkerArgs_obj TriggerConfig_obj "worker-spell"] ret-75)
                                                                     )
                                                                     (new $array-inline-24
                                                                      (seq
                                                                       (seq
                                                                        (ap "created deal spell" $array-inline-24)
                                                                        (ap ret-75 $array-inline-24)
                                                                       )
                                                                       (canon worker_id-0_gate.$.[0] $array-inline-24  #array-inline-24-0)
                                                                      )
                                                                     )
                                                                    )
                                                                    (new $-ephemeral-stream-
                                                                     (new #-ephemeral-canon-
                                                                      (canon -relay- $-ephemeral-stream-  #-ephemeral-canon-)
                                                                     )
                                                                    )
                                                                   )
                                                                   (call %init_peer_id% ("op" "concat_strings") ["decider deal_id=" log-16-0_flat_flat ": "] ret-76)
                                                                  )
                                                                  (par
                                                                   (call %init_peer_id% ("run-console" "print") [ret-76 #array-inline-24-0])
                                                                   (null)
                                                                  )
                                                                 )
                                                                 (call %init_peer_id% ("json" "stringify") [#array-inline-24-0] ret-77)
                                                                )
                                                                (call %init_peer_id% ("op" "concat_strings") [ret-76 ret-77] ret-78)
                                                               )
                                                               (call %init_peer_id% (-spell_id-arg- "store_log") [ret-78] ret-79)
                                                              )
                                                             )
                                                             (seq
                                                              (seq
                                                               (seq
                                                                (seq
                                                                 (seq
                                                                  (seq
                                                                   (seq
                                                                    (seq
                                                                     (seq
                                                                      (new $array-inline-25
                                                                       (seq
                                                                        (seq
                                                                         (ap "error installing deal spell" $array-inline-25)
                                                                         (ap :error: $array-inline-25)
                                                                        )
                                                                        (canon worker_id-0_gate.$.[0] $array-inline-25  #array-inline-25-0)
                                                                       )
                                                                      )
                                                                      (call worker_id-0_gate.$.[0] ("json" "stringify") [#array-inline-25-0] ret-80)
                                                                     )
                                                                     (ap ret-80 $error-0)
                                                                    )
                                                                    (new $array-inline-26
                                                                     (seq
                                                                      (seq
                                                                       (ap "error installing deal spell" $array-inline-26)
                                                                       (ap :error: $array-inline-26)
                                                                      )
                                                                      (canon worker_id-0_gate.$.[0] $array-inline-26  #array-inline-26-0)
                                                                     )
                                                                    )
                                                                   )
                                                                   (new $-ephemeral-stream-
                                                                    (new #-ephemeral-canon-
                                                                     (canon -relay- $-ephemeral-stream-  #-ephemeral-canon-)
                                                                    )
                                                                   )
                                                                  )
                                                                  (call %init_peer_id% ("op" "concat_strings") ["decider deal_id=" log-16-0_flat_flat ": "] ret-81)
                                                                 )
                                                                 (par
                                                                  (call %init_peer_id% ("run-console" "print") [ret-81 #array-inline-26-0])
                                                                  (null)
                                                                 )
                                                                )
                                                                (call %init_peer_id% ("json" "stringify") [#array-inline-26-0] ret-82)
                                                               )
                                                               (call %init_peer_id% ("op" "concat_strings") [ret-81 ret-82] ret-83)
                                                              )
                                                              (call %init_peer_id% (-spell_id-arg- "store_log") [ret-83] ret-84)
                                                             )
                                                            )
                                                            (seq
                                                             (seq
                                                              (seq
                                                               (seq
                                                                (seq
                                                                 (seq
                                                                  (seq
                                                                   (seq
                                                                    (seq
                                                                     (new $array-inline-27
                                                                      (seq
                                                                       (seq
                                                                        (ap "cannot create deal spell" $array-inline-27)
                                                                        (ap :error:.$.message $array-inline-27)
                                                                       )
                                                                       (canon worker_id-0_gate.$.[0] $array-inline-27  #array-inline-27-0)
                                                                      )
                                                                     )
                                                                     (call worker_id-0_gate.$.[0] ("json" "stringify") [#array-inline-27-0] ret-85)
                                                                    )
                                                                    (ap ret-85 $error-0)
                                                                   )
                                                                   (new $array-inline-28
                                                                    (seq
                                                                     (seq
                                                                      (ap "cannot create deal spell, deal join failed" $array-inline-28)
                                                                      (ap :error:.$.message $array-inline-28)
                                                                     )
                                                                     (canon worker_id-0_gate.$.[0] $array-inline-28  #array-inline-28-0)
                                                                    )
                                                                   )
                                                                  )
                                                                  (new $-ephemeral-stream-
                                                                   (new #-ephemeral-canon-
                                                                    (canon -relay- $-ephemeral-stream-  #-ephemeral-canon-)
                                                                   )
                                                                  )
                                                                 )
                                                                 (call %init_peer_id% ("op" "concat_strings") ["decider deal_id=" log-16-0_flat_flat ": "] ret-86)
                                                                )
                                                                (par
                                                                 (call %init_peer_id% ("run-console" "print") [ret-86 #array-inline-28-0])
                                                                 (null)
                                                                )
                                                               )
                                                               (call %init_peer_id% ("json" "stringify") [#array-inline-28-0] ret-87)
                                                              )
                                                              (call %init_peer_id% ("op" "concat_strings") [ret-86 ret-87] ret-88)
                                                             )
                                                             (call %init_peer_id% (-spell_id-arg- "store_log") [ret-88] ret-89)
                                                            )
                                                           )
                                                          )
                                                          (seq
                                                           (seq
                                                            (new $-ephemeral-stream-
                                                             (new #-ephemeral-canon-
                                                              (canon -relay- $-ephemeral-stream-  #-ephemeral-canon-)
                                                             )
                                                            )
                                                            (new $-ephemeral-stream-
                                                             (new #-ephemeral-canon-
                                                              (canon %init_peer_id% $-ephemeral-stream-  #-ephemeral-canon-)
                                                             )
                                                            )
                                                           )
                                                           (fail :error:)
                                                          )
                                                         )
                                                        )
                                                        (xor
                                                         (call %init_peer_id% ("worker" "deactivate") [log-16-0_flat_flat])
                                                         (seq
                                                          (seq
                                                           (seq
                                                            (seq
                                                             (seq
                                                              (seq
                                                               (seq
                                                                (new $array-inline-29
                                                                 (seq
                                                                  (seq
                                                                   (ap "error deactivating worker" $array-inline-29)
                                                                   (ap :error: $array-inline-29)
                                                                  )
                                                                  (canon %init_peer_id% $array-inline-29  #array-inline-29-0)
                                                                 )
                                                                )
                                                                (call %init_peer_id% ("json" "stringify") [#array-inline-29-0] ret-90)
                                                               )
                                                               (ap ret-90 $error-0)
                                                              )
                                                              (call %init_peer_id% ("op" "concat_strings") ["decider deal_id=" log-16-0_flat_flat ": "] ret-91)
                                                             )
                                                             (par
                                                              (call %init_peer_id% ("run-console" "print") [ret-91 #array-inline-29-0])
                                                              (null)
                                                             )
                                                            )
                                                            (call %init_peer_id% ("json" "stringify") [#array-inline-29-0] ret-92)
                                                           )
                                                           (call %init_peer_id% ("op" "concat_strings") [ret-91 ret-92] ret-93)
                                                          )
                                                          (call %init_peer_id% (-spell_id-arg- "store_log") [ret-93] ret-94)
                                                         )
                                                        )
                                                       )
                                                      )
                                                      (seq
                                                       (ap :error: -if-error-)
                                                       (xor
                                                        (match :error:.$.error_code 10001
                                                         (null)
                                                        )
                                                        (fail -if-error-)
                                                       )
                                                      )
                                                     )
                                                    )
                                                   )
                                                  )
                                                  (seq
                                                   (ap :error: -if-error-)
                                                   (xor
                                                    (match :error:.$.error_code 10002
                                                     (null)
                                                    )
                                                    (fail -if-error-)
                                                   )
                                                  )
                                                 )
                                                )
                                               )
                                               (canon %init_peer_id% $worker_id-0  #-worker_id-fix-0)
                                              )
                                              (ap #-worker_id-fix-0 -worker_id-flat-0)
                                             )
                                             (canon %init_peer_id% $error-0  #-error-fix-0)
                                            )
                                            (ap #-error-fix-0 -error-flat-0)
                                           )
                                          )
                                         )
                                        )
                                        (new -if-else-error-
                                         (new -else-error-
                                          (new -if-error-
                                           (xor
                                            (match -worker_id-flat-0 []
                                             (seq
                                              (seq
                                               (seq
                                                (seq
                                                 (seq
                                                  (seq
                                                   (new %InstallationFailed_obj_map
                                                    (seq
                                                     (ap ("log" log-16-0) %InstallationFailed_obj_map)
                                                     (canon %init_peer_id% %InstallationFailed_obj_map  InstallationFailed_obj)
                                                    )
                                                   )
                                                   (new %FailedDealError_obj_map
                                                    (seq
                                                     (seq
                                                      (ap ("content" InstallationFailed_obj) %FailedDealError_obj_map)
                                                      (ap ("type" "InstallationFailed") %FailedDealError_obj_map)
                                                     )
                                                     (canon %init_peer_id% %FailedDealError_obj_map  FailedDealError_obj)
                                                    )
                                                   )
                                                  )
                                                  (new %FailedDeal_obj_map
                                                   (seq
                                                    (seq
                                                     (seq
                                                      (ap ("deal_id" log-16-0_flat_flat) %FailedDeal_obj_map)
                                                      (ap ("message" -error-flat-0.$.[0]) %FailedDeal_obj_map)
                                                     )
                                                     (ap ("payload" FailedDealError_obj) %FailedDeal_obj_map)
                                                    )
                                                    (canon %init_peer_id% %FailedDeal_obj_map  FailedDeal_obj)
                                                   )
                                                  )
                                                 )
                                                 (call %init_peer_id% ("json" "stringify") [FailedDeal_obj] ret-95)
                                                )
                                                (call %init_peer_id% (-spell_id-arg- "list_push_string") ["failed_deals" ret-95] ret-96)
                                               )
                                               (xor
                                                (match ret-96.$.success true
                                                 (ap false not-6)
                                                )
                                                (ap true not-6)
                                               )
                                              )
                                              (new -if-error-
                                               (xor
                                                (match not-6 true
                                                 (seq
                                                  (seq
                                                   (seq
                                                    (new $array-inline-30
                                                     (seq
                                                      (seq
                                                       (seq
                                                        (seq
                                                         (ap "can't store value in list by key" $array-inline-30)
                                                         (ap "failed_deals" $array-inline-30)
                                                        )
                                                        (ap "error:" $array-inline-30)
                                                       )
                                                       (ap ret-96.$.error $array-inline-30)
                                                      )
                                                      (canon %init_peer_id% $array-inline-30  #array-inline-30-0)
                                                     )
                                                    )
                                                    (par
                                                     (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-30-0])
                                                     (null)
                                                    )
                                                   )
                                                   (call %init_peer_id% ("json" "stringify") [#array-inline-30-0] ret-97)
                                                  )
                                                  (call %init_peer_id% (-spell_id-arg- "store_log") [ret-97] ret-98)
                                                 )
                                                )
                                                (seq
                                                 (ap :error: -if-error-)
                                                 (xor
                                                  (match :error:.$.error_code 10001
                                                   (null)
                                                  )
                                                  (fail -if-error-)
                                                 )
                                                )
                                               )
                                              )
                                             )
                                            )
                                            (seq
                                             (ap :error: -if-error-)
                                             (xor
                                              (match :error:.$.error_code 10001
                                               (seq
                                                (seq
                                                 (seq
                                                  (seq
                                                   (seq
                                                    (seq
                                                     (seq
                                                      (seq
                                                       (call %init_peer_id% ("chain_connector" "register_worker") [log-16-0.$.info.unit_id -worker_id-flat-0.$.[0] -chain-arg- log-16-0_flat_flat] ret-99)
                                                       (ap ret-99.$.success ret-99_flat)
                                                      )
                                                      (new -if-else-error-
                                                       (new -else-error-
                                                        (new -if-error-
                                                         (xor
                                                          (match ret-99.$.success true
                                                           (seq
                                                            (seq
                                                             (seq
                                                              (seq
                                                               (seq
                                                                (seq
                                                                 (seq
                                                                  (seq
                                                                   (seq
                                                                    (seq
                                                                     (new $array-inline-31
                                                                      (seq
                                                                       (seq
                                                                        (ap "registered worker tx_hash" $array-inline-31)
                                                                        (ap ret-99.$.tx_hash $array-inline-31)
                                                                       )
                                                                       (canon %init_peer_id% $array-inline-31  #array-inline-31-0)
                                                                      )
                                                                     )
                                                                     (call %init_peer_id% ("op" "concat_strings") ["decider deal_id=" log-16-0_flat_flat ": "] ret-100)
                                                                    )
                                                                    (par
                                                                     (call %init_peer_id% ("run-console" "print") [ret-100 #array-inline-31-0])
                                                                     (null)
                                                                    )
                                                                   )
                                                                   (call %init_peer_id% ("json" "stringify") [#array-inline-31-0] ret-101)
                                                                  )
                                                                  (call %init_peer_id% ("op" "concat_strings") [ret-100 ret-101] ret-102)
                                                                 )
                                                                 (call %init_peer_id% (-spell_id-arg- "store_log") [ret-102] ret-103)
                                                                )
                                                                (new %WorkerTxInfo_obj_map
                                                                 (seq
                                                                  (seq
                                                                   (ap ("deal_id" log-16-0_flat_flat) %WorkerTxInfo_obj_map)
                                                                   (ap ("tx_hash" ret-99.$.tx_hash.[0]) %WorkerTxInfo_obj_map)
                                                                  )
                                                                  (canon %init_peer_id% %WorkerTxInfo_obj_map  WorkerTxInfo_obj)
                                                                 )
                                                                )
                                                               )
                                                               (call %init_peer_id% ("json" "stringify") [WorkerTxInfo_obj] ret-104)
                                                              )
                                                              (call %init_peer_id% (-spell_id-arg- "list_push_string") ["worker_registration_txs" ret-104] ret-105)
                                                             )
                                                             (xor
                                                              (match ret-105.$.success true
                                                               (ap false not-7)
                                                              )
                                                              (ap true not-7)
                                                             )
                                                            )
                                                            (new -if-error-
                                                             (xor
                                                              (match not-7 true
                                                               (seq
                                                                (seq
                                                                 (seq
                                                                  (new $array-inline-32
                                                                   (seq
                                                                    (seq
                                                                     (seq
                                                                      (seq
                                                                       (ap "can't store value in list by key" $array-inline-32)
                                                                       (ap "worker_registration_txs" $array-inline-32)
                                                                      )
                                                                      (ap "error:" $array-inline-32)
                                                                     )
                                                                     (ap ret-105.$.error $array-inline-32)
                                                                    )
                                                                    (canon %init_peer_id% $array-inline-32  #array-inline-32-0)
                                                                   )
                                                                  )
                                                                  (par
                                                                   (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-32-0])
                                                                   (null)
                                                                  )
                                                                 )
                                                                 (call %init_peer_id% ("json" "stringify") [#array-inline-32-0] ret-106)
                                                                )
                                                                (call %init_peer_id% (-spell_id-arg- "store_log") [ret-106] ret-107)
                                                               )
                                                              )
                                                              (seq
                                                               (ap :error: -if-error-)
                                                               (xor
                                                                (match :error:.$.error_code 10001
                                                                 (null)
                                                                )
                                                                (fail -if-error-)
                                                               )
                                                              )
                                                             )
                                                            )
                                                           )
                                                          )
                                                          (seq
                                                           (ap :error: -if-error-)
                                                           (xor
                                                            (match :error:.$.error_code 10001
                                                             (seq
                                                              (seq
                                                               (seq
                                                                (seq
                                                                 (seq
                                                                  (seq
                                                                   (seq
                                                                    (seq
                                                                     (seq
                                                                      (seq
                                                                       (seq
                                                                        (seq
                                                                         (new $array-inline-33
                                                                          (seq
                                                                           (seq
                                                                            (ap "error registering worker" $array-inline-33)
                                                                            (ap ret-99.$.error.[0] $array-inline-33)
                                                                           )
                                                                           (canon %init_peer_id% $array-inline-33  #array-inline-33-0)
                                                                          )
                                                                         )
                                                                         (call %init_peer_id% ("op" "concat_strings") ["decider deal_id=" log-16-0_flat_flat ": "] ret-108)
                                                                        )
                                                                        (par
                                                                         (call %init_peer_id% ("run-console" "print") [ret-108 #array-inline-33-0])
                                                                         (null)
                                                                        )
                                                                       )
                                                                       (call %init_peer_id% ("json" "stringify") [#array-inline-33-0] ret-109)
                                                                      )
                                                                      (call %init_peer_id% ("op" "concat_strings") [ret-108 ret-109] ret-110)
                                                                     )
                                                                     (call %init_peer_id% (-spell_id-arg- "store_log") [ret-110] ret-111)
                                                                    )
                                                                    (new %TxFailed_obj_map
                                                                     (seq
                                                                      (seq
                                                                       (ap ("block_number" []) %TxFailed_obj_map)
                                                                       (ap ("tx_hash" []) %TxFailed_obj_map)
                                                                      )
                                                                      (canon %init_peer_id% %TxFailed_obj_map  TxFailed_obj)
                                                                     )
                                                                    )
                                                                   )
                                                                   (new %FailedDealError_obj-0_map
                                                                    (seq
                                                                     (seq
                                                                      (ap ("content" TxFailed_obj) %FailedDealError_obj-0_map)
                                                                      (ap ("type" "TxFailed") %FailedDealError_obj-0_map)
                                                                     )
                                                                     (canon %init_peer_id% %FailedDealError_obj-0_map  FailedDealError_obj-0)
                                                                    )
                                                                   )
                                                                  )
                                                                  (new %FailedDeal_obj-0_map
                                                                   (seq
                                                                    (seq
                                                                     (seq
                                                                      (ap ("deal_id" log-16-0_flat_flat) %FailedDeal_obj-0_map)
                                                                      (ap ("message" ret-99.$.error.[0]) %FailedDeal_obj-0_map)
                                                                     )
                                                                     (ap ("payload" FailedDealError_obj-0) %FailedDeal_obj-0_map)
                                                                    )
                                                                    (canon %init_peer_id% %FailedDeal_obj-0_map  FailedDeal_obj-0)
                                                                   )
                                                                  )
                                                                 )
                                                                 (call %init_peer_id% ("json" "stringify") [FailedDeal_obj-0] ret-112)
                                                                )
                                                                (call %init_peer_id% (-spell_id-arg- "list_push_string") ["failed_deals" ret-112] ret-113)
                                                               )
                                                               (xor
                                                                (match ret-113.$.success true
                                                                 (ap false not-8)
                                                                )
                                                                (ap true not-8)
                                                               )
                                                              )
                                                              (new -if-error-
                                                               (xor
                                                                (match not-8 true
                                                                 (seq
                                                                  (seq
                                                                   (seq
                                                                    (new $array-inline-34
                                                                     (seq
                                                                      (seq
                                                                       (seq
                                                                        (seq
                                                                         (ap "can't store value in list by key" $array-inline-34)
                                                                         (ap "failed_deals" $array-inline-34)
                                                                        )
                                                                        (ap "error:" $array-inline-34)
                                                                       )
                                                                       (ap ret-113.$.error $array-inline-34)
                                                                      )
                                                                      (canon %init_peer_id% $array-inline-34  #array-inline-34-0)
                                                                     )
                                                                    )
                                                                    (par
                                                                     (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-34-0])
                                                                     (null)
                                                                    )
                                                                   )
                                                                   (call %init_peer_id% ("json" "stringify") [#array-inline-34-0] ret-114)
                                                                  )
                                                                  (call %init_peer_id% (-spell_id-arg- "store_log") [ret-114] ret-115)
                                                                 )
                                                                )
                                                                (seq
                                                                 (ap :error: -if-error-)
                                                                 (xor
                                                                  (match :error:.$.error_code 10001
                                                                   (null)
                                                                  )
                                                                  (fail -if-error-)
                                                                 )
                                                                )
                                                               )
                                                              )
                                                             )
                                                            )
                                                            (seq
                                                             (seq
                                                              (ap :error: -else-error-)
                                                              (xor
                                                               (match :error:.$.error_code 10001
                                                                (ap -if-error- -if-else-error-)
                                                               )
                                                               (ap -else-error- -if-else-error-)
                                                              )
                                                             )
                                                             (fail -if-else-error-)
                                                            )
                                                           )
                                                          )
                                                         )
                                                        )
                                                       )
                                                      )
                                                     )
                                                     (new -if-error-
                                                      (xor
                                                       (match ret-99_flat true
                                                        (seq
                                                         (xor
                                                          (seq
                                                           (seq
                                                            (seq
                                                             (seq
                                                              (seq
                                                               (seq
                                                                (seq
                                                                 (seq
                                                                  (seq
                                                                   (seq
                                                                    (seq
                                                                     (seq
                                                                      (seq
                                                                       (seq
                                                                        (new %JoinedDeal_obj_map
                                                                         (seq
                                                                          (seq
                                                                           (ap ("deal_id" log-16-0_flat_flat) %JoinedDeal_obj_map)
                                                                           (ap ("worker_id" -worker_id-flat-0.$.[0]) %JoinedDeal_obj_map)
                                                                          )
                                                                          (canon %init_peer_id% %JoinedDeal_obj_map  JoinedDeal_obj)
                                                                         )
                                                                        )
                                                                        (call %init_peer_id% ("json" "stringify") [JoinedDeal_obj] ret-116)
                                                                       )
                                                                       (call %init_peer_id% (-spell_id-arg- "list_push_string") ["joined_deals" ret-116] ret-117)
                                                                      )
                                                                      (new %DealState_obj_map
                                                                       (seq
                                                                        (ap ("left_boundary" log-16-0.$.block_number) %DealState_obj_map)
                                                                        (canon %init_peer_id% %DealState_obj_map  DealState_obj)
                                                                       )
                                                                      )
                                                                     )
                                                                     (call %init_peer_id% ("json" "stringify") [DealState_obj] ret-118)
                                                                    )
                                                                    (call %init_peer_id% (-spell_id-arg- "set_string") [log-16-0_flat_flat ret-118] ret-119)
                                                                   )
                                                                   (new %DealState_obj-0_map
                                                                    (seq
                                                                     (ap ("left_boundary" log-16-0.$.block_number) %DealState_obj-0_map)
                                                                     (canon %init_peer_id% %DealState_obj-0_map  DealState_obj-0)
                                                                    )
                                                                   )
                                                                  )
                                                                  (call %init_peer_id% ("json" "stringify") [DealState_obj-0] ret-120)
                                                                 )
                                                                 (call %init_peer_id% ("op" "concat_strings") ["removed_state:" log-16-0_flat_flat] ret-121)
                                                                )
                                                                (call %init_peer_id% (-spell_id-arg- "set_string") [ret-121 ret-120] ret-122)
                                                               )
                                                               (call %init_peer_id% ("op" "concat_strings") ["decider deal_id=" log-16-0_flat_flat ": "] ret-123)
                                                              )
                                                              (par
                                                               (call %init_peer_id% ("run-console" "print") [ret-123 "deal state saved to kv"])
                                                               (null)
                                                              )
                                                             )
                                                             (call %init_peer_id% ("json" "stringify") ["deal state saved to kv"] ret-124)
                                                            )
                                                            (call %init_peer_id% ("op" "concat_strings") [ret-123 ret-124] ret-125)
                                                           )
                                                           (call %init_peer_id% (-spell_id-arg- "store_log") [ret-125] ret-126)
                                                          )
                                                          (seq
                                                           (seq
                                                            (seq
                                                             (seq
                                                              (seq
                                                               (new $array-inline-35
                                                                (seq
                                                                 (seq
                                                                  (ap "cannot store deal state, deal join failed" $array-inline-35)
                                                                  (ap :error:.$.message $array-inline-35)
                                                                 )
                                                                 (canon %init_peer_id% $array-inline-35  #array-inline-35-0)
                                                                )
                                                               )
                                                               (call %init_peer_id% ("op" "concat_strings") ["decider deal_id=" log-16-0_flat_flat ": "] ret-127)
                                                              )
                                                              (par
                                                               (call %init_peer_id% ("run-console" "print") [ret-127 #array-inline-35-0])
                                                               (null)
                                                              )
                                                             )
                                                             (call %init_peer_id% ("json" "stringify") [#array-inline-35-0] ret-128)
                                                            )
                                                            (call %init_peer_id% ("op" "concat_strings") [ret-127 ret-128] ret-129)
                                                           )
                                                           (call %init_peer_id% (-spell_id-arg- "store_log") [ret-129] ret-130)
                                                          )
                                                         )
                                                         (ap log-16-0_flat_flat $deal_ids)
                                                        )
                                                       )
                                                       (seq
                                                        (ap :error: -if-error-)
                                                        (xor
                                                         (match :error:.$.error_code 10001
                                                          (null)
                                                         )
                                                         (fail -if-error-)
                                                        )
                                                       )
                                                      )
                                                     )
                                                    )
                                                    (call %init_peer_id% ("chain_connector" "hex_sub") [log-16-0.$.block_number 1] ret-131)
                                                   )
                                                   (xor
                                                    (match ret-131.$.success true
                                                     (ap false not-9)
                                                    )
                                                    (ap true not-9)
                                                   )
                                                  )
                                                  (new -if-error-
                                                   (xor
                                                    (match not-9 true
                                                     (seq
                                                      (seq
                                                       (seq
                                                        (new $array-inline-36
                                                         (seq
                                                          (seq
                                                           (seq
                                                            (ap "hex_sub failed" $array-inline-36)
                                                            (ap log-16-0.$.block_number $array-inline-36)
                                                           )
                                                           (ap 1 $array-inline-36)
                                                          )
                                                          (canon %init_peer_id% $array-inline-36  #array-inline-36-0)
                                                         )
                                                        )
                                                        (par
                                                         (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-36-0])
                                                         (null)
                                                        )
                                                       )
                                                       (call %init_peer_id% ("json" "stringify") [#array-inline-36-0] ret-132)
                                                      )
                                                      (call %init_peer_id% (-spell_id-arg- "store_log") [ret-132] ret-133)
                                                     )
                                                    )
                                                    (seq
                                                     (ap :error: -if-error-)
                                                     (xor
                                                      (match :error:.$.error_code 10001
                                                       (null)
                                                      )
                                                      (fail -if-error-)
                                                     )
                                                    )
                                                   )
                                                  )
                                                 )
                                                 (ap ret-131.$.diff ret-131_flat)
                                                )
                                                (new -if-error-
                                                 (xor
                                                  (mismatch ret-131_flat []
                                                   (seq
                                                    (seq
                                                     (new $result-6
                                                      (seq
                                                       (seq
                                                        (seq
                                                         (call %init_peer_id% ("chain_connector" "hex_cmp") [ret-131_flat.$.[0] -left-flat-0.$.[0]] ret-134)
                                                         (new -if-else-error-
                                                          (new -else-error-
                                                           (new -if-error-
                                                            (xor
                                                             (match ret-134.$.success true
                                                              (seq
                                                               (call %init_peer_id% ("cmp" "gt") [ret-134.$.ordering 0] gt)
                                                               (ap gt $result-6)
                                                              )
                                                             )
                                                             (seq
                                                              (ap :error: -if-error-)
                                                              (xor
                                                               (match :error:.$.error_code 10001
                                                                (seq
                                                                 (seq
                                                                  (seq
                                                                   (new $array-inline-37
                                                                    (seq
                                                                     (seq
                                                                      (ap "hex_cmp error" $array-inline-37)
                                                                      (ap ret-134.$.error $array-inline-37)
                                                                     )
                                                                     (canon %init_peer_id% $array-inline-37  #array-inline-37-0)
                                                                    )
                                                                   )
                                                                   (par
                                                                    (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-37-0])
                                                                    (null)
                                                                   )
                                                                  )
                                                                  (call %init_peer_id% ("json" "stringify") [#array-inline-37-0] ret-135)
                                                                 )
                                                                 (call %init_peer_id% (-spell_id-arg- "store_log") [ret-135] ret-136)
                                                                )
                                                               )
                                                               (seq
                                                                (seq
                                                                 (ap :error: -else-error-)
                                                                 (xor
                                                                  (match :error:.$.error_code 10001
                                                                   (ap -if-error- -if-else-error-)
                                                                  )
                                                                  (ap -else-error- -if-else-error-)
                                                                 )
                                                                )
                                                                (fail -if-else-error-)
                                                               )
                                                              )
                                                             )
                                                            )
                                                           )
                                                          )
                                                         )
                                                        )
                                                        (canon %init_peer_id% $result-6  #-result-fix-0-1)
                                                       )
                                                       (ap #-result-fix-0-1 -result-flat-0-1)
                                                      )
                                                     )
                                                     (new $array-inline-38
                                                      (seq
                                                       (ap true $array-inline-38)
                                                       (canon %init_peer_id% $array-inline-38  #array-inline-38-0)
                                                      )
                                                     )
                                                    )
                                                    (new -if-error-
                                                     (xor
                                                      (match -result-flat-0-1 #array-inline-38-0
                                                       (seq
                                                        (seq
                                                         (seq
                                                          (seq
                                                           (seq
                                                            (seq
                                                             (seq
                                                              (seq
                                                               (new $array-inline-39
                                                                (seq
                                                                 (seq
                                                                  (ap "gt_set: will set last seen to" $array-inline-39)
                                                                  (ap ret-131_flat.$.[0] $array-inline-39)
                                                                 )
                                                                 (canon %init_peer_id% $array-inline-39  #array-inline-39-0)
                                                                )
                                                               )
                                                               (par
                                                                (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-39-0])
                                                                (null)
                                                               )
                                                              )
                                                              (call %init_peer_id% ("json" "stringify") [#array-inline-39-0] ret-137)
                                                             )
                                                             (call %init_peer_id% (-spell_id-arg- "store_log") [ret-137] ret-138)
                                                            )
                                                            (new $array-inline-40
                                                             (seq
                                                              (seq
                                                               (ap "will set last seen to" $array-inline-40)
                                                               (ap ret-131_flat.$.[0] $array-inline-40)
                                                              )
                                                              (canon %init_peer_id% $array-inline-40  #array-inline-40-0)
                                                             )
                                                            )
                                                           )
                                                           (par
                                                            (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-40-0])
                                                            (null)
                                                           )
                                                          )
                                                          (call %init_peer_id% ("json" "stringify") [#array-inline-40-0] ret-139)
                                                         )
                                                         (call %init_peer_id% (-spell_id-arg- "store_log") [ret-139] ret-140)
                                                        )
                                                        (xor
                                                         (seq
                                                          (call %init_peer_id% (-spell_id-arg- "set_string") ["last_seen_block" ret-131_flat.$.[0]] ret-141)
                                                          (new -if-else-error-
                                                           (new -else-error-
                                                            (new -if-error-
                                                             (xor
                                                              (match ret-141.$.success true
                                                               (seq
                                                                (seq
                                                                 (seq
                                                                  (new $array-inline-41
                                                                   (seq
                                                                    (seq
                                                                     (ap "saved last seen" $array-inline-41)
                                                                     (ap ret-131_flat.$.[0] $array-inline-41)
                                                                    )
                                                                    (canon %init_peer_id% $array-inline-41  #array-inline-41-0)
                                                                   )
                                                                  )
                                                                  (par
                                                                   (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-41-0])
                                                                   (null)
                                                                  )
                                                                 )
                                                                 (call %init_peer_id% ("json" "stringify") [#array-inline-41-0] ret-142)
                                                                )
                                                                (call %init_peer_id% (-spell_id-arg- "store_log") [ret-142] ret-143)
                                                               )
                                                              )
                                                              (seq
                                                               (ap :error: -if-error-)
                                                               (xor
                                                                (match :error:.$.error_code 10001
                                                                 (seq
                                                                  (seq
                                                                   (seq
                                                                    (new $array-inline-42
                                                                     (seq
                                                                      (seq
                                                                       (ap "error saving last_seen_block" $array-inline-42)
                                                                       (ap ret-141.$.error $array-inline-42)
                                                                      )
                                                                      (canon %init_peer_id% $array-inline-42  #array-inline-42-0)
                                                                     )
                                                                    )
                                                                    (par
                                                                     (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-42-0])
                                                                     (null)
                                                                    )
                                                                   )
                                                                   (call %init_peer_id% ("json" "stringify") [#array-inline-42-0] ret-144)
                                                                  )
                                                                  (call %init_peer_id% (-spell_id-arg- "store_log") [ret-144] ret-145)
                                                                 )
                                                                )
                                                                (seq
                                                                 (seq
                                                                  (ap :error: -else-error-)
                                                                  (xor
                                                                   (match :error:.$.error_code 10001
                                                                    (ap -if-error- -if-else-error-)
                                                                   )
                                                                   (ap -else-error- -if-else-error-)
                                                                  )
                                                                 )
                                                                 (fail -if-else-error-)
                                                                )
                                                               )
                                                              )
                                                             )
                                                            )
                                                           )
                                                          )
                                                         )
                                                         (seq
                                                          (seq
                                                           (seq
                                                            (new $array-inline-43
                                                             (seq
                                                              (seq
                                                               (ap "error saving last_seen_block" $array-inline-43)
                                                               (ap :error: $array-inline-43)
                                                              )
                                                              (canon %init_peer_id% $array-inline-43  #array-inline-43-0)
                                                             )
                                                            )
                                                            (par
                                                             (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-43-0])
                                                             (null)
                                                            )
                                                           )
                                                           (call %init_peer_id% ("json" "stringify") [#array-inline-43-0] ret-146)
                                                          )
                                                          (call %init_peer_id% (-spell_id-arg- "store_log") [ret-146] ret-147)
                                                         )
                                                        )
                                                       )
                                                      )
                                                      (seq
                                                       (ap :error: -if-error-)
                                                       (xor
                                                        (match :error:.$.error_code 10001
                                                         (null)
                                                        )
                                                        (fail -if-error-)
                                                       )
                                                      )
                                                     )
                                                    )
                                                   )
                                                  )
                                                  (seq
                                                   (ap :error: -if-error-)
                                                   (xor
                                                    (match :error:.$.error_code 10002
                                                     (null)
                                                    )
                                                    (fail -if-error-)
                                                   )
                                                  )
                                                 )
                                                )
                                               )
                                              )
                                              (seq
                                               (seq
                                                (ap :error: -else-error-)
                                                (xor
                                                 (match :error:.$.error_code 10001
                                                  (ap -if-error- -if-else-error-)
                                                 )
                                                 (ap -else-error- -if-else-error-)
                                                )
                                               )
                                               (fail -if-else-error-)
                                              )
                                             )
                                            )
                                           )
                                          )
                                         )
                                        )
                                       )
                                       (next log-16-0)
                                      )
                                      (null)
                                     )
                                     (canon %init_peer_id% $deal_ids  #deal_ids_canon)
                                    )
                                    (new -if-error-
                                     (xor
                                      (mismatch #deal_ids_canon []
                                       (seq
                                        (seq
                                         (seq
                                          (seq
                                           (seq
                                            (seq
                                             (seq
                                              (new $array-inline-44
                                               (seq
                                                (seq
                                                 (seq
                                                  (seq
                                                   (seq
                                                    (canon %init_peer_id% $deal_ids  #deal_ids_to_functor)
                                                    (ap #deal_ids_to_functor.length deal_ids_length)
                                                   )
                                                   (ap "polling statuses of " $array-inline-44)
                                                  )
                                                  (ap deal_ids_length $array-inline-44)
                                                 )
                                                 (ap "deals" $array-inline-44)
                                                )
                                                (canon %init_peer_id% $array-inline-44  #array-inline-44-0)
                                               )
                                              )
                                              (par
                                               (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-44-0])
                                               (null)
                                              )
                                             )
                                             (call %init_peer_id% ("json" "stringify") [#array-inline-44-0] ret-148)
                                            )
                                            (call %init_peer_id% (-spell_id-arg- "store_log") [ret-148] ret-149)
                                           )
                                           (canon %init_peer_id% $deal_ids  #deal_ids_canon-0)
                                          )
                                          (call %init_peer_id% ("chain_connector" "get_status_batch") [-chain-arg-.$.api_endpoint #deal_ids_canon-0] ret-150)
                                         )
                                         (xor
                                          (match ret-150.$.success true
                                           (ap false not-10)
                                          )
                                          (ap true not-10)
                                         )
                                        )
                                        (new -if-else-error-
                                         (new -else-error-
                                          (new -if-error-
                                           (xor
                                            (match not-10 true
                                             (seq
                                              (seq
                                               (seq
                                                (new $array-inline-45
                                                 (seq
                                                  (seq
                                                   (ap "couldn't obtain deal statuses, error: " $array-inline-45)
                                                   (ap ret-150.$.error.[0] $array-inline-45)
                                                  )
                                                  (canon %init_peer_id% $array-inline-45  #array-inline-45-0)
                                                 )
                                                )
                                                (par
                                                 (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-45-0])
                                                 (null)
                                                )
                                               )
                                               (call %init_peer_id% ("json" "stringify") [#array-inline-45-0] ret-151)
                                              )
                                              (call %init_peer_id% (-spell_id-arg- "store_log") [ret-151] ret-152)
                                             )
                                            )
                                            (seq
                                             (ap :error: -if-error-)
                                             (xor
                                              (match :error:.$.error_code 10001
                                               (seq
                                                (seq
                                                 (seq
                                                  (seq
                                                   (new $array-inline-46
                                                    (seq
                                                     (seq
                                                      (seq
                                                       (seq
                                                        (seq
                                                         (seq
                                                          (ap ret-150.$.statuses ret-150_flat)
                                                          (ap ret-150_flat ret-150_flat_to_functor)
                                                         )
                                                         (ap ret-150_flat_to_functor.length ret-150_flat_length)
                                                        )
                                                        (ap "found statuses for" $array-inline-46)
                                                       )
                                                       (ap ret-150_flat_length $array-inline-46)
                                                      )
                                                      (ap "deals" $array-inline-46)
                                                     )
                                                     (canon %init_peer_id% $array-inline-46  #array-inline-46-0)
                                                    )
                                                   )
                                                   (par
                                                    (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-46-0])
                                                    (null)
                                                   )
                                                  )
                                                  (call %init_peer_id% ("json" "stringify") [#array-inline-46-0] ret-153)
                                                 )
                                                 (call %init_peer_id% (-spell_id-arg- "store_log") [ret-153] ret-154)
                                                )
                                                (fold ret-150.$.statuses deal_status-0
                                                 (seq
                                                  (seq
                                                   (xor
                                                    (match deal_status-0.$.success true
                                                     (ap false not-11)
                                                    )
                                                    (ap true not-11)
                                                   )
                                                   (new -if-else-error-
                                                    (new -else-error-
                                                     (new -if-error-
                                                      (xor
                                                       (match not-11 true
                                                        (seq
                                                         (seq
                                                          (seq
                                                           (new $array-inline-47
                                                            (seq
                                                             (seq
                                                              (ap "couldn't obtain deal status, error:" $array-inline-47)
                                                              (ap deal_status-0.$.error.[0] $array-inline-47)
                                                             )
                                                             (canon %init_peer_id% $array-inline-47  #array-inline-47-0)
                                                            )
                                                           )
                                                           (par
                                                            (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-47-0])
                                                            (null)
                                                           )
                                                          )
                                                          (call %init_peer_id% ("json" "stringify") [#array-inline-47-0] ret-155)
                                                         )
                                                         (call %init_peer_id% (-spell_id-arg- "store_log") [ret-155] ret-156)
                                                        )
                                                       )
                                                       (seq
                                                        (ap :error: -if-error-)
                                                        (xor
                                                         (match :error:.$.error_code 10001
                                                          (new -if-else-error-
                                                           (new -else-error-
                                                            (new -if-error-
                                                             (xor
                                                              (match deal_status-0.$.status "ACTIVE"
                                                               (xor
                                                                (seq
                                                                 (seq
                                                                  (call %init_peer_id% ("worker" "is_active") [deal_status-0.$.deal_id] ret-157)
                                                                  (xor
                                                                   (match ret-157 true
                                                                    (ap false not-12)
                                                                   )
                                                                   (ap true not-12)
                                                                  )
                                                                 )
                                                                 (new -if-error-
                                                                  (xor
                                                                   (match not-12 true
                                                                    (seq
                                                                     (seq
                                                                      (seq
                                                                       (seq
                                                                        (seq
                                                                         (call %init_peer_id% ("op" "concat_strings") ["decider deal_id=" deal_status-0.$.deal_id ": "] ret-158)
                                                                         (par
                                                                          (call %init_peer_id% ("run-console" "print") [ret-158 "activating worker"])
                                                                          (null)
                                                                         )
                                                                        )
                                                                        (call %init_peer_id% ("json" "stringify") ["activating worker"] ret-159)
                                                                       )
                                                                       (call %init_peer_id% ("op" "concat_strings") [ret-158 ret-159] ret-160)
                                                                      )
                                                                      (call %init_peer_id% (-spell_id-arg- "store_log") [ret-160] ret-161)
                                                                     )
                                                                     (call %init_peer_id% ("worker" "activate") [deal_status-0.$.deal_id])
                                                                    )
                                                                   )
                                                                   (seq
                                                                    (ap :error: -if-error-)
                                                                    (xor
                                                                     (match :error:.$.error_code 10001
                                                                      (null)
                                                                     )
                                                                     (fail -if-error-)
                                                                    )
                                                                   )
                                                                  )
                                                                 )
                                                                )
                                                                (seq
                                                                 (seq
                                                                  (seq
                                                                   (seq
                                                                    (seq
                                                                     (new $array-inline-48
                                                                      (seq
                                                                       (seq
                                                                        (ap "couldn't activate a worker" $array-inline-48)
                                                                        (ap :error: $array-inline-48)
                                                                       )
                                                                       (canon %init_peer_id% $array-inline-48  #array-inline-48-0)
                                                                      )
                                                                     )
                                                                     (call %init_peer_id% ("op" "concat_strings") ["decider deal_id=" deal_status-0.$.deal_id ": "] ret-162)
                                                                    )
                                                                    (par
                                                                     (call %init_peer_id% ("run-console" "print") [ret-162 #array-inline-48-0])
                                                                     (null)
                                                                    )
                                                                   )
                                                                   (call %init_peer_id% ("json" "stringify") [#array-inline-48-0] ret-163)
                                                                  )
                                                                  (call %init_peer_id% ("op" "concat_strings") [ret-162 ret-163] ret-164)
                                                                 )
                                                                 (call %init_peer_id% (-spell_id-arg- "store_log") [ret-164] ret-165)
                                                                )
                                                               )
                                                              )
                                                              (seq
                                                               (ap :error: -if-error-)
                                                               (xor
                                                                (match :error:.$.error_code 10001
                                                                 (new -if-else-error-
                                                                  (new -else-error-
                                                                   (new -if-error-
                                                                    (xor
                                                                     (match deal_status-0.$.status "INACTIVE"
                                                                      (xor
                                                                       (seq
                                                                        (call %init_peer_id% ("worker" "is_active") [deal_status-0.$.deal_id] ret-166)
                                                                        (new -if-error-
                                                                         (xor
                                                                          (match ret-166 true
                                                                           (seq
                                                                            (seq
                                                                             (seq
                                                                              (seq
                                                                               (seq
                                                                                (call %init_peer_id% ("op" "concat_strings") ["decider deal_id=" deal_status-0.$.deal_id ": "] ret-167)
                                                                                (par
                                                                                 (call %init_peer_id% ("run-console" "print") [ret-167 "deactivating worker"])
                                                                                 (null)
                                                                                )
                                                                               )
                                                                               (call %init_peer_id% ("json" "stringify") ["deactivating worker"] ret-168)
                                                                              )
                                                                              (call %init_peer_id% ("op" "concat_strings") [ret-167 ret-168] ret-169)
                                                                             )
                                                                             (call %init_peer_id% (-spell_id-arg- "store_log") [ret-169] ret-170)
                                                                            )
                                                                            (call %init_peer_id% ("worker" "deactivate") [deal_status-0.$.deal_id])
                                                                           )
                                                                          )
                                                                          (seq
                                                                           (ap :error: -if-error-)
                                                                           (xor
                                                                            (match :error:.$.error_code 10001
                                                                             (null)
                                                                            )
                                                                            (fail -if-error-)
                                                                           )
                                                                          )
                                                                         )
                                                                        )
                                                                       )
                                                                       (seq
                                                                        (seq
                                                                         (seq
                                                                          (seq
                                                                           (seq
                                                                            (new $array-inline-49
                                                                             (seq
                                                                              (seq
                                                                               (ap "couldn't deactivate a worker" $array-inline-49)
                                                                               (ap :error: $array-inline-49)
                                                                              )
                                                                              (canon %init_peer_id% $array-inline-49  #array-inline-49-0)
                                                                             )
                                                                            )
                                                                            (call %init_peer_id% ("op" "concat_strings") ["decider deal_id=" deal_status-0.$.deal_id ": "] ret-171)
                                                                           )
                                                                           (par
                                                                            (call %init_peer_id% ("run-console" "print") [ret-171 #array-inline-49-0])
                                                                            (null)
                                                                           )
                                                                          )
                                                                          (call %init_peer_id% ("json" "stringify") [#array-inline-49-0] ret-172)
                                                                         )
                                                                         (call %init_peer_id% ("op" "concat_strings") [ret-171 ret-172] ret-173)
                                                                        )
                                                                        (call %init_peer_id% (-spell_id-arg- "store_log") [ret-173] ret-174)
                                                                       )
                                                                      )
                                                                     )
                                                                     (seq
                                                                      (ap :error: -if-error-)
                                                                      (xor
                                                                       (match :error:.$.error_code 10001
                                                                        (new -if-else-error-
                                                                         (new -else-error-
                                                                          (new -if-error-
                                                                           (xor
                                                                            (match deal_status-0.$.status "ENDED"
                                                                             (seq
                                                                              (call %init_peer_id% ("worker" "get_worker_id") [deal_status-0.$.deal_id] ret-175)
                                                                              (new -if-else-error-
                                                                               (new -else-error-
                                                                                (new -if-error-
                                                                                 (xor
                                                                                  (mismatch ret-175 []
                                                                                   (seq
                                                                                    (seq
                                                                                     (seq
                                                                                      (seq
                                                                                       (seq
                                                                                        (seq
                                                                                         (call %init_peer_id% ("op" "concat_strings") ["decider deal_id=" deal_status-0.$.deal_id ": "] ret-176)
                                                                                         (par
                                                                                          (call %init_peer_id% ("run-console" "print") [ret-176 "removing the worker and the deal state from kv"])
                                                                                          (null)
                                                                                         )
                                                                                        )
                                                                                        (call %init_peer_id% ("json" "stringify") ["removing the worker and the deal state from kv"] ret-177)
                                                                                       )
                                                                                       (call %init_peer_id% ("op" "concat_strings") [ret-176 ret-177] ret-178)
                                                                                      )
                                                                                      (call %init_peer_id% (-spell_id-arg- "store_log") [ret-178] ret-179)
                                                                                     )
                                                                                     (xor
                                                                                      (call %init_peer_id% ("worker" "remove") [ret-175.$.[0]])
                                                                                      (seq
                                                                                       (seq
                                                                                        (seq
                                                                                         (seq
                                                                                          (seq
                                                                                           (new $array-inline-50
                                                                                            (seq
                                                                                             (seq
                                                                                              (seq
                                                                                               (ap "couldn't remove a worker" $array-inline-50)
                                                                                               (ap ret-175.$.[0] $array-inline-50)
                                                                                              )
                                                                                              (ap :error: $array-inline-50)
                                                                                             )
                                                                                             (canon %init_peer_id% $array-inline-50  #array-inline-50-0)
                                                                                            )
                                                                                           )
                                                                                           (call %init_peer_id% ("op" "concat_strings") ["decider deal_id=" deal_status-0.$.deal_id ": "] ret-180)
                                                                                          )
                                                                                          (par
                                                                                           (call %init_peer_id% ("run-console" "print") [ret-180 #array-inline-50-0])
                                                                                           (null)
                                                                                          )
                                                                                         )
                                                                                         (call %init_peer_id% ("json" "stringify") [#array-inline-50-0] ret-181)
                                                                                        )
                                                                                        (call %init_peer_id% ("op" "concat_strings") [ret-180 ret-181] ret-182)
                                                                                       )
                                                                                       (call %init_peer_id% (-spell_id-arg- "store_log") [ret-182] ret-183)
                                                                                      )
                                                                                     )
                                                                                    )
                                                                                    (xor
                                                                                     (seq
                                                                                      (seq
                                                                                       (seq
                                                                                        (seq
                                                                                         (seq
                                                                                          (new %JoinedDeal_obj-0_map
                                                                                           (seq
                                                                                            (seq
                                                                                             (ap ("deal_id" deal_status-0.$.deal_id) %JoinedDeal_obj-0_map)
                                                                                             (ap ("worker_id" ret-175.$.[0]) %JoinedDeal_obj-0_map)
                                                                                            )
                                                                                            (canon %init_peer_id% %JoinedDeal_obj-0_map  JoinedDeal_obj-0)
                                                                                           )
                                                                                          )
                                                                                          (call %init_peer_id% ("json" "stringify") [JoinedDeal_obj-0] ret-184)
                                                                                         )
                                                                                         (call %init_peer_id% (-spell_id-arg- "list_remove_string") ["joined_deals" ret-184] ret-185)
                                                                                        )
                                                                                        (call %init_peer_id% (-spell_id-arg- "remove_key") [deal_status-0.$.deal_id] ret-186)
                                                                                       )
                                                                                       (call %init_peer_id% ("op" "concat_strings") ["removed_state:" deal_status-0.$.deal_id] ret-187)
                                                                                      )
                                                                                      (call %init_peer_id% (-spell_id-arg- "remove_key") [ret-187] ret-188)
                                                                                     )
                                                                                     (seq
                                                                                      (seq
                                                                                       (seq
                                                                                        (seq
                                                                                         (seq
                                                                                          (new $array-inline-51
                                                                                           (seq
                                                                                            (seq
                                                                                             (ap "couldn't remove the deal" $array-inline-51)
                                                                                             (ap :error: $array-inline-51)
                                                                                            )
                                                                                            (canon %init_peer_id% $array-inline-51  #array-inline-51-0)
                                                                                           )
                                                                                          )
                                                                                          (call %init_peer_id% ("op" "concat_strings") ["decider deal_id=" deal_status-0.$.deal_id ": "] ret-189)
                                                                                         )
                                                                                         (par
                                                                                          (call %init_peer_id% ("run-console" "print") [ret-189 #array-inline-51-0])
                                                                                          (null)
                                                                                         )
                                                                                        )
                                                                                        (call %init_peer_id% ("json" "stringify") [#array-inline-51-0] ret-190)
                                                                                       )
                                                                                       (call %init_peer_id% ("op" "concat_strings") [ret-189 ret-190] ret-191)
                                                                                      )
                                                                                      (call %init_peer_id% (-spell_id-arg- "store_log") [ret-191] ret-192)
                                                                                     )
                                                                                    )
                                                                                   )
                                                                                  )
                                                                                  (seq
                                                                                   (ap :error: -if-error-)
                                                                                   (xor
                                                                                    (match :error:.$.error_code 10002
                                                                                     (seq
                                                                                      (seq
                                                                                       (seq
                                                                                        (seq
                                                                                         (call %init_peer_id% ("op" "concat_strings") ["decider deal_id=" deal_status-0.$.deal_id ": "] ret-193)
                                                                                         (par
                                                                                          (call %init_peer_id% ("run-console" "print") [ret-193 "deal doesn't have associated worker O.o"])
                                                                                          (null)
                                                                                         )
                                                                                        )
                                                                                        (call %init_peer_id% ("json" "stringify") ["deal doesn't have associated worker O.o"] ret-194)
                                                                                       )
                                                                                       (call %init_peer_id% ("op" "concat_strings") [ret-193 ret-194] ret-195)
                                                                                      )
                                                                                      (call %init_peer_id% (-spell_id-arg- "store_log") [ret-195] ret-196)
                                                                                     )
                                                                                    )
                                                                                    (seq
                                                                                     (seq
                                                                                      (ap :error: -else-error-)
                                                                                      (xor
                                                                                       (match :error:.$.error_code 10001
                                                                                        (ap -if-error- -if-else-error-)
                                                                                       )
                                                                                       (ap -else-error- -if-else-error-)
                                                                                      )
                                                                                     )
                                                                                     (fail -if-else-error-)
                                                                                    )
                                                                                   )
                                                                                  )
                                                                                 )
                                                                                )
                                                                               )
                                                                              )
                                                                             )
                                                                            )
                                                                            (seq
                                                                             (ap :error: -if-error-)
                                                                             (xor
                                                                              (match :error:.$.error_code 10001
                                                                               (seq
                                                                                (seq
                                                                                 (seq
                                                                                  (new $array-inline-52
                                                                                   (seq
                                                                                    (seq
                                                                                     (ap "unsupported deal status: " $array-inline-52)
                                                                                     (ap deal_status-0 $array-inline-52)
                                                                                    )
                                                                                    (canon %init_peer_id% $array-inline-52  #array-inline-52-0)
                                                                                   )
                                                                                  )
                                                                                  (par
                                                                                   (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-52-0])
                                                                                   (null)
                                                                                  )
                                                                                 )
                                                                                 (call %init_peer_id% ("json" "stringify") [#array-inline-52-0] ret-197)
                                                                                )
                                                                                (call %init_peer_id% (-spell_id-arg- "store_log") [ret-197] ret-198)
                                                                               )
                                                                              )
                                                                              (seq
                                                                               (seq
                                                                                (ap :error: -else-error-)
                                                                                (xor
                                                                                 (match :error:.$.error_code 10001
                                                                                  (ap -if-error- -if-else-error-)
                                                                                 )
                                                                                 (ap -else-error- -if-else-error-)
                                                                                )
                                                                               )
                                                                               (fail -if-else-error-)
                                                                              )
                                                                             )
                                                                            )
                                                                           )
                                                                          )
                                                                         )
                                                                        )
                                                                       )
                                                                       (seq
                                                                        (seq
                                                                         (ap :error: -else-error-)
                                                                         (xor
                                                                          (match :error:.$.error_code 10001
                                                                           (ap -if-error- -if-else-error-)
                                                                          )
                                                                          (ap -else-error- -if-else-error-)
                                                                         )
                                                                        )
                                                                        (fail -if-else-error-)
                                                                       )
                                                                      )
                                                                     )
                                                                    )
                                                                   )
                                                                  )
                                                                 )
                                                                )
                                                                (seq
                                                                 (seq
                                                                  (ap :error: -else-error-)
                                                                  (xor
                                                                   (match :error:.$.error_code 10001
                                                                    (ap -if-error- -if-else-error-)
                                                                   )
                                                                   (ap -else-error- -if-else-error-)
                                                                  )
                                                                 )
                                                                 (fail -if-else-error-)
                                                                )
                                                               )
                                                              )
                                                             )
                                                            )
                                                           )
                                                          )
                                                         )
                                                         (seq
                                                          (seq
                                                           (ap :error: -else-error-)
                                                           (xor
                                                            (match :error:.$.error_code 10001
                                                             (ap -if-error- -if-else-error-)
                                                            )
                                                            (ap -else-error- -if-else-error-)
                                                           )
                                                          )
                                                          (fail -if-else-error-)
                                                         )
                                                        )
                                                       )
                                                      )
                                                     )
                                                    )
                                                   )
                                                  )
                                                  (next deal_status-0)
                                                 )
                                                 (null)
                                                )
                                               )
                                              )
                                              (seq
                                               (seq
                                                (ap :error: -else-error-)
                                                (xor
                                                 (match :error:.$.error_code 10001
                                                  (ap -if-error- -if-else-error-)
                                                 )
                                                 (ap -else-error- -if-else-error-)
                                                )
                                               )
                                               (fail -if-else-error-)
                                              )
                                             )
                                            )
                                           )
                                          )
                                         )
                                        )
                                       )
                                      )
                                      (seq
                                       (ap :error: -if-error-)
                                       (xor
                                        (match :error:.$.error_code 10002
                                         (null)
                                        )
                                        (fail -if-error-)
                                       )
                                      )
                                     )
                                    )
                                   )
                                  )
                                 )
                                 (new $array-inline-53
                                  (seq
                                   (seq
                                    (seq
                                     (ap "poll_new_deals call Hex.min" $array-inline-53)
                                     (ap -latest-flat-0.$.[0] $array-inline-53)
                                    )
                                    (ap -poll-flat-0.$.[0].right_boundary $array-inline-53)
                                   )
                                   (canon %init_peer_id% $array-inline-53  #array-inline-53-0)
                                  )
                                 )
                                )
                                (par
                                 (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-53-0])
                                 (null)
                                )
                               )
                               (call %init_peer_id% ("json" "stringify") [#array-inline-53-0] ret-199)
                              )
                              (call %init_peer_id% (-spell_id-arg- "store_log") [ret-199] ret-200)
                             )
                             (call %init_peer_id% ("chain_connector" "hex_min") [-latest-flat-0.$.[0] -poll-flat-0.$.[0].right_boundary] ret-201)
                            )
                            (xor
                             (match ret-201.$.success true
                              (ap false not-13)
                             )
                             (ap true not-13)
                            )
                           )
                           (new -if-error-
                            (xor
                             (match not-13 true
                              (seq
                               (seq
                                (seq
                                 (new $array-inline-54
                                  (seq
                                   (seq
                                    (seq
                                     (ap "hex_min failed" $array-inline-54)
                                     (ap -latest-flat-0.$.[0] $array-inline-54)
                                    )
                                    (ap -poll-flat-0.$.[0].right_boundary $array-inline-54)
                                   )
                                   (canon %init_peer_id% $array-inline-54  #array-inline-54-0)
                                  )
                                 )
                                 (par
                                  (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-54-0])
                                  (null)
                                 )
                                )
                                (call %init_peer_id% ("json" "stringify") [#array-inline-54-0] ret-202)
                               )
                               (call %init_peer_id% (-spell_id-arg- "store_log") [ret-202] ret-203)
                              )
                             )
                             (seq
                              (ap :error: -if-error-)
                              (xor
                               (match :error:.$.error_code 10001
                                (null)
                               )
                               (fail -if-error-)
                              )
                             )
                            )
                           )
                          )
                          (ap ret-201.$.hex ret-201_flat)
                         )
                         (new -if-error-
                          (xor
                           (mismatch ret-201_flat []
                            (seq
                             (new $result-8
                              (seq
                               (seq
                                (seq
                                 (seq
                                  (call %init_peer_id% (-spell_id-arg- "get_string") ["last_seen_block"] ret-204)
                                  (xor
                                   (match ret-204.$.success true
                                    (ap false not-14)
                                   )
                                   (ap true not-14)
                                  )
                                 )
                                 (new -if-else-error-
                                  (new -else-error-
                                   (new -if-error-
                                    (xor
                                     (match not-14 true
                                      (seq
                                       (seq
                                        (seq
                                         (new $array-inline-55
                                          (seq
                                           (seq
                                            (seq
                                             (ap "get_string failed" $array-inline-55)
                                             (ap "last_seen_block" $array-inline-55)
                                            )
                                            (ap ret-204.$.error $array-inline-55)
                                           )
                                           (canon %init_peer_id% $array-inline-55  #array-inline-55-0)
                                          )
                                         )
                                         (par
                                          (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-55-0])
                                          (null)
                                         )
                                        )
                                        (call %init_peer_id% ("json" "stringify") [#array-inline-55-0] ret-205)
                                       )
                                       (call %init_peer_id% (-spell_id-arg- "store_log") [ret-205] ret-206)
                                      )
                                     )
                                     (seq
                                      (ap :error: -if-error-)
                                      (xor
                                       (match :error:.$.error_code 10001
                                        (seq
                                         (xor
                                          (match ret-204.$.absent true
                                           (ap false not-15)
                                          )
                                          (ap true not-15)
                                         )
                                         (new -if-error-
                                          (xor
                                           (match not-15 true
                                            (ap ret-204.$.value $result-8)
                                           )
                                           (seq
                                            (ap :error: -if-error-)
                                            (xor
                                             (match :error:.$.error_code 10001
                                              (null)
                                             )
                                             (fail -if-error-)
                                            )
                                           )
                                          )
                                         )
                                        )
                                       )
                                       (seq
                                        (seq
                                         (ap :error: -else-error-)
                                         (xor
                                          (match :error:.$.error_code 10001
                                           (ap -if-error- -if-else-error-)
                                          )
                                          (ap -else-error- -if-else-error-)
                                         )
                                        )
                                        (fail -if-else-error-)
                                       )
                                      )
                                     )
                                    )
                                   )
                                  )
                                 )
                                )
                                (canon %init_peer_id% $result-8  #-result-fix-0-2)
                               )
                               (ap #-result-fix-0-2 -result-flat-0-2)
                              )
                             )
                             (new -if-else-error-
                              (new -else-error-
                               (new -if-error-
                                (xor
                                 (mismatch -result-flat-0-2 []
                                  (seq
                                   (seq
                                    (new $result-9
                                     (seq
                                      (seq
                                       (seq
                                        (call %init_peer_id% ("chain_connector" "hex_cmp") [ret-201_flat.$.[0] -result-flat-0-2.$.[0]] ret-207)
                                        (new -if-else-error-
                                         (new -else-error-
                                          (new -if-error-
                                           (xor
                                            (match ret-207.$.success true
                                             (seq
                                              (call %init_peer_id% ("cmp" "gt") [ret-207.$.ordering 0] gt-0)
                                              (ap gt-0 $result-9)
                                             )
                                            )
                                            (seq
                                             (ap :error: -if-error-)
                                             (xor
                                              (match :error:.$.error_code 10001
                                               (seq
                                                (seq
                                                 (seq
                                                  (new $array-inline-56
                                                   (seq
                                                    (seq
                                                     (ap "hex_cmp error" $array-inline-56)
                                                     (ap ret-207.$.error $array-inline-56)
                                                    )
                                                    (canon %init_peer_id% $array-inline-56  #array-inline-56-0)
                                                   )
                                                  )
                                                  (par
                                                   (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-56-0])
                                                   (null)
                                                  )
                                                 )
                                                 (call %init_peer_id% ("json" "stringify") [#array-inline-56-0] ret-208)
                                                )
                                                (call %init_peer_id% (-spell_id-arg- "store_log") [ret-208] ret-209)
                                               )
                                              )
                                              (seq
                                               (seq
                                                (ap :error: -else-error-)
                                                (xor
                                                 (match :error:.$.error_code 10001
                                                  (ap -if-error- -if-else-error-)
                                                 )
                                                 (ap -else-error- -if-else-error-)
                                                )
                                               )
                                               (fail -if-else-error-)
                                              )
                                             )
                                            )
                                           )
                                          )
                                         )
                                        )
                                       )
                                       (canon %init_peer_id% $result-9  #-result-fix-0-3)
                                      )
                                      (ap #-result-fix-0-3 -result-flat-0-3)
                                     )
                                    )
                                    (new $array-inline-57
                                     (seq
                                      (ap true $array-inline-57)
                                      (canon %init_peer_id% $array-inline-57  #array-inline-57-0)
                                     )
                                    )
                                   )
                                   (new -if-error-
                                    (xor
                                     (match -result-flat-0-3 #array-inline-57-0
                                      (seq
                                       (seq
                                        (seq
                                         (seq
                                          (seq
                                           (seq
                                            (seq
                                             (seq
                                              (new $array-inline-58
                                               (seq
                                                (seq
                                                 (ap "gt_set: will set last seen to" $array-inline-58)
                                                 (ap ret-201_flat.$.[0] $array-inline-58)
                                                )
                                                (canon %init_peer_id% $array-inline-58  #array-inline-58-0)
                                               )
                                              )
                                              (par
                                               (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-58-0])
                                               (null)
                                              )
                                             )
                                             (call %init_peer_id% ("json" "stringify") [#array-inline-58-0] ret-210)
                                            )
                                            (call %init_peer_id% (-spell_id-arg- "store_log") [ret-210] ret-211)
                                           )
                                           (new $array-inline-59
                                            (seq
                                             (seq
                                              (ap "will set last seen to" $array-inline-59)
                                              (ap ret-201_flat.$.[0] $array-inline-59)
                                             )
                                             (canon %init_peer_id% $array-inline-59  #array-inline-59-0)
                                            )
                                           )
                                          )
                                          (par
                                           (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-59-0])
                                           (null)
                                          )
                                         )
                                         (call %init_peer_id% ("json" "stringify") [#array-inline-59-0] ret-212)
                                        )
                                        (call %init_peer_id% (-spell_id-arg- "store_log") [ret-212] ret-213)
                                       )
                                       (xor
                                        (seq
                                         (call %init_peer_id% (-spell_id-arg- "set_string") ["last_seen_block" ret-201_flat.$.[0]] ret-214)
                                         (new -if-else-error-
                                          (new -else-error-
                                           (new -if-error-
                                            (xor
                                             (match ret-214.$.success true
                                              (seq
                                               (seq
                                                (seq
                                                 (new $array-inline-60
                                                  (seq
                                                   (seq
                                                    (ap "saved last seen" $array-inline-60)
                                                    (ap ret-201_flat.$.[0] $array-inline-60)
                                                   )
                                                   (canon %init_peer_id% $array-inline-60  #array-inline-60-0)
                                                  )
                                                 )
                                                 (par
                                                  (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-60-0])
                                                  (null)
                                                 )
                                                )
                                                (call %init_peer_id% ("json" "stringify") [#array-inline-60-0] ret-215)
                                               )
                                               (call %init_peer_id% (-spell_id-arg- "store_log") [ret-215] ret-216)
                                              )
                                             )
                                             (seq
                                              (ap :error: -if-error-)
                                              (xor
                                               (match :error:.$.error_code 10001
                                                (seq
                                                 (seq
                                                  (seq
                                                   (new $array-inline-61
                                                    (seq
                                                     (seq
                                                      (ap "error saving last_seen_block" $array-inline-61)
                                                      (ap ret-214.$.error $array-inline-61)
                                                     )
                                                     (canon %init_peer_id% $array-inline-61  #array-inline-61-0)
                                                    )
                                                   )
                                                   (par
                                                    (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-61-0])
                                                    (null)
                                                   )
                                                  )
                                                  (call %init_peer_id% ("json" "stringify") [#array-inline-61-0] ret-217)
                                                 )
                                                 (call %init_peer_id% (-spell_id-arg- "store_log") [ret-217] ret-218)
                                                )
                                               )
                                               (seq
                                                (seq
                                                 (ap :error: -else-error-)
                                                 (xor
                                                  (match :error:.$.error_code 10001
                                                   (ap -if-error- -if-else-error-)
                                                  )
                                                  (ap -else-error- -if-else-error-)
                                                 )
                                                )
                                                (fail -if-else-error-)
                                               )
                                              )
                                             )
                                            )
                                           )
                                          )
                                         )
                                        )
                                        (seq
                                         (seq
                                          (seq
                                           (new $array-inline-62
                                            (seq
                                             (seq
                                              (ap "error saving last_seen_block" $array-inline-62)
                                              (ap :error: $array-inline-62)
                                             )
                                             (canon %init_peer_id% $array-inline-62  #array-inline-62-0)
                                            )
                                           )
                                           (par
                                            (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-62-0])
                                            (null)
                                           )
                                          )
                                          (call %init_peer_id% ("json" "stringify") [#array-inline-62-0] ret-219)
                                         )
                                         (call %init_peer_id% (-spell_id-arg- "store_log") [ret-219] ret-220)
                                        )
                                       )
                                      )
                                     )
                                     (seq
                                      (ap :error: -if-error-)
                                      (xor
                                       (match :error:.$.error_code 10001
                                        (null)
                                       )
                                       (fail -if-error-)
                                      )
                                     )
                                    )
                                   )
                                  )
                                 )
                                 (seq
                                  (ap :error: -if-error-)
                                  (xor
                                   (match :error:.$.error_code 10002
                                    (seq
                                     (seq
                                      (seq
                                       (seq
                                        (seq
                                         (seq
                                          (seq
                                           (seq
                                            (new $array-inline-63
                                             (seq
                                              (seq
                                               (ap "increase: will set last seen to" $array-inline-63)
                                               (ap ret-201_flat.$.[0] $array-inline-63)
                                              )
                                              (canon %init_peer_id% $array-inline-63  #array-inline-63-0)
                                             )
                                            )
                                            (par
                                             (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-63-0])
                                             (null)
                                            )
                                           )
                                           (call %init_peer_id% ("json" "stringify") [#array-inline-63-0] ret-221)
                                          )
                                          (call %init_peer_id% (-spell_id-arg- "store_log") [ret-221] ret-222)
                                         )
                                         (new $array-inline-64
                                          (seq
                                           (seq
                                            (ap "will set last seen to" $array-inline-64)
                                            (ap ret-201_flat.$.[0] $array-inline-64)
                                           )
                                           (canon %init_peer_id% $array-inline-64  #array-inline-64-0)
                                          )
                                         )
                                        )
                                        (par
                                         (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-64-0])
                                         (null)
                                        )
                                       )
                                       (call %init_peer_id% ("json" "stringify") [#array-inline-64-0] ret-223)
                                      )
                                      (call %init_peer_id% (-spell_id-arg- "store_log") [ret-223] ret-224)
                                     )
                                     (xor
                                      (seq
                                       (call %init_peer_id% (-spell_id-arg- "set_string") ["last_seen_block" ret-201_flat.$.[0]] ret-225)
                                       (new -if-else-error-
                                        (new -else-error-
                                         (new -if-error-
                                          (xor
                                           (match ret-225.$.success true
                                            (seq
                                             (seq
                                              (seq
                                               (new $array-inline-65
                                                (seq
                                                 (seq
                                                  (ap "saved last seen" $array-inline-65)
                                                  (ap ret-201_flat.$.[0] $array-inline-65)
                                                 )
                                                 (canon %init_peer_id% $array-inline-65  #array-inline-65-0)
                                                )
                                               )
                                               (par
                                                (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-65-0])
                                                (null)
                                               )
                                              )
                                              (call %init_peer_id% ("json" "stringify") [#array-inline-65-0] ret-226)
                                             )
                                             (call %init_peer_id% (-spell_id-arg- "store_log") [ret-226] ret-227)
                                            )
                                           )
                                           (seq
                                            (ap :error: -if-error-)
                                            (xor
                                             (match :error:.$.error_code 10001
                                              (seq
                                               (seq
                                                (seq
                                                 (new $array-inline-66
                                                  (seq
                                                   (seq
                                                    (ap "error saving last_seen_block" $array-inline-66)
                                                    (ap ret-225.$.error $array-inline-66)
                                                   )
                                                   (canon %init_peer_id% $array-inline-66  #array-inline-66-0)
                                                  )
                                                 )
                                                 (par
                                                  (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-66-0])
                                                  (null)
                                                 )
                                                )
                                                (call %init_peer_id% ("json" "stringify") [#array-inline-66-0] ret-228)
                                               )
                                               (call %init_peer_id% (-spell_id-arg- "store_log") [ret-228] ret-229)
                                              )
                                             )
                                             (seq
                                              (seq
                                               (ap :error: -else-error-)
                                               (xor
                                                (match :error:.$.error_code 10001
                                                 (ap -if-error- -if-else-error-)
                                                )
                                                (ap -else-error- -if-else-error-)
                                               )
                                              )
                                              (fail -if-else-error-)
                                             )
                                            )
                                           )
                                          )
                                         )
                                        )
                                       )
                                      )
                                      (seq
                                       (seq
                                        (seq
                                         (new $array-inline-67
                                          (seq
                                           (seq
                                            (ap "error saving last_seen_block" $array-inline-67)
                                            (ap :error: $array-inline-67)
                                           )
                                           (canon %init_peer_id% $array-inline-67  #array-inline-67-0)
                                          )
                                         )
                                         (par
                                          (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-67-0])
                                          (null)
                                         )
                                        )
                                        (call %init_peer_id% ("json" "stringify") [#array-inline-67-0] ret-230)
                                       )
                                       (call %init_peer_id% (-spell_id-arg- "store_log") [ret-230] ret-231)
                                      )
                                     )
                                    )
                                   )
                                   (seq
                                    (seq
                                     (ap :error: -else-error-)
                                     (xor
                                      (match :error:.$.error_code 10001
                                       (ap -if-error- -if-else-error-)
                                      )
                                      (ap -else-error- -if-else-error-)
                                     )
                                    )
                                    (fail -if-else-error-)
                                   )
                                  )
                                 )
                                )
                               )
                              )
                             )
                            )
                           )
                           (seq
                            (ap :error: -if-error-)
                            (xor
                             (match :error:.$.error_code 10002
                              (null)
                             )
                             (fail -if-error-)
                            )
                           )
                          )
                         )
                        )
                       )
                       (seq
                        (ap :error: -if-error-)
                        (xor
                         (match :error:.$.error_code 10002
                          (null)
                         )
                         (fail -if-error-)
                        )
                       )
                      )
                     )
                    )
                   )
                   (seq
                    (ap :error: -if-error-)
                    (xor
                     (match :error:.$.error_code 10002
                      (seq
                       (seq
                        (seq
                         (new $array-inline-68
                          (seq
                           (ap "get_left_boundary returns nil, unable to proceed" $array-inline-68)
                           (canon %init_peer_id% $array-inline-68  #array-inline-68-0)
                          )
                         )
                         (par
                          (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-68-0])
                          (null)
                         )
                        )
                        (call %init_peer_id% ("json" "stringify") [#array-inline-68-0] ret-232)
                       )
                       (call %init_peer_id% (-spell_id-arg- "store_log") [ret-232] ret-233)
                      )
                     )
                     (seq
                      (seq
                       (ap :error: -else-error-)
                       (xor
                        (match :error:.$.error_code 10001
                         (ap -if-error- -if-else-error-)
                        )
                        (ap -else-error- -if-else-error-)
                       )
                      )
                      (fail -if-else-error-)
                     )
                    )
                   )
                  )
                 )
                )
               )
              )
              (null)
             )
            )
            (xor
             (seq
              (new $changes-0
               (seq
                (seq
                 (seq
                  (seq
                   (seq
                    (new $requests
                     (seq
                      (seq
                       (new -if-error-
                        (xor
                         (mismatch -joined_deals-flat-0 []
                          (new $deal_updates
                           (fold -joined_deals-flat-0 joined_deal-0-0
                            (seq
                             (seq
                              (seq
                               (ap joined_deal-0-0.$.deal_id joined_deal-0-0_flat)
                               (new $state-1
                                (seq
                                 (seq
                                  (seq
                                   (call %init_peer_id% (-spell_id-arg- "get_string") [joined_deal-0-0_flat] ret-234)
                                   (xor
                                    (match ret-234.$.success false
                                     (ap ret-234.$.success and)
                                    )
                                    (seq
                                     (xor
                                      (match ret-234.$.absent true
                                       (ap false not-16)
                                      )
                                      (ap true not-16)
                                     )
                                     (ap not-16 and)
                                    )
                                   )
                                  )
                                  (new -if-else-error-
                                   (new -else-error-
                                    (new -if-error-
                                     (xor
                                      (match and true
                                       (seq
                                        (new $state-2
                                         (seq
                                          (seq
                                           (xor
                                            (seq
                                             (call %init_peer_id% ("json" "parse") [ret-234.$.value] ret-235)
                                             (ap ret-235 $state-2)
                                            )
                                            (seq
                                             (seq
                                              (seq
                                               (seq
                                                (seq
                                                 (new $array-inline-69
                                                  (seq
                                                   (seq
                                                    (ap "failed to parse deal state from json" $array-inline-69)
                                                    (ap :error: $array-inline-69)
                                                   )
                                                   (canon %init_peer_id% $array-inline-69  #array-inline-69-0)
                                                  )
                                                 )
                                                 (call %init_peer_id% ("op" "concat_strings") ["decider deal_id=" joined_deal-0-0_flat ": "] ret-236)
                                                )
                                                (par
                                                 (call %init_peer_id% ("run-console" "print") [ret-236 #array-inline-69-0])
                                                 (null)
                                                )
                                               )
                                               (call %init_peer_id% ("json" "stringify") [#array-inline-69-0] ret-237)
                                              )
                                              (call %init_peer_id% ("op" "concat_strings") [ret-236 ret-237] ret-238)
                                             )
                                             (call %init_peer_id% (-spell_id-arg- "store_log") [ret-238] ret-239)
                                            )
                                           )
                                           (canon %init_peer_id% $state-2  #-state-fix-0)
                                          )
                                          (ap #-state-fix-0 -state-flat-0)
                                         )
                                        )
                                        (ap -state-flat-0 $state-1)
                                       )
                                      )
                                      (seq
                                       (ap :error: -if-error-)
                                       (xor
                                        (match :error:.$.error_code 10001
                                         (seq
                                          (seq
                                           (seq
                                            (seq
                                             (seq
                                              (seq
                                               (new $array-inline-70
                                                (seq
                                                 (seq
                                                  (ap "deal state not found:" $array-inline-70)
                                                  (ap ret-234.$.error $array-inline-70)
                                                 )
                                                 (canon %init_peer_id% $array-inline-70  #array-inline-70-0)
                                                )
                                               )
                                               (call %init_peer_id% ("op" "concat_strings") ["decider deal_id=" joined_deal-0-0_flat ": "] ret-240)
                                              )
                                              (par
                                               (call %init_peer_id% ("run-console" "print") [ret-240 #array-inline-70-0])
                                               (null)
                                              )
                                             )
                                             (call %init_peer_id% ("json" "stringify") [#array-inline-70-0] ret-241)
                                            )
                                            (call %init_peer_id% ("op" "concat_strings") [ret-240 ret-241] ret-242)
                                           )
                                           (call %init_peer_id% (-spell_id-arg- "store_log") [ret-242] ret-243)
                                          )
                                          (ap [] $state-1)
                                         )
                                        )
                                        (seq
                                         (seq
                                          (ap :error: -else-error-)
                                          (xor
                                           (match :error:.$.error_code 10001
                                            (ap -if-error- -if-else-error-)
                                           )
                                           (ap -else-error- -if-else-error-)
                                          )
                                         )
                                         (fail -if-else-error-)
                                        )
                                       )
                                      )
                                     )
                                    )
                                   )
                                  )
                                 )
                                 (new $state-1_test
                                  (seq
                                   (seq
                                    (fold $state-1 state-1_fold_var
                                     (seq
                                      (seq
                                       (ap state-1_fold_var $state-1_test)
                                       (canon %init_peer_id% $state-1_test  #state-1_iter_canon)
                                      )
                                      (xor
                                       (match #state-1_iter_canon.length 1
                                        (null)
                                       )
                                       (next state-1_fold_var)
                                      )
                                     )
                                     (never)
                                    )
                                    (canon %init_peer_id% $state-1_test  #state-1_result_canon)
                                   )
                                   (ap #state-1_result_canon state-1_gate)
                                  )
                                 )
                                )
                               )
                              )
                              (new -if-error-
                               (xor
                                (mismatch state-1_gate.$.[0] []
                                 (seq
                                  (seq
                                   (new %DealInfo_obj_map
                                    (seq
                                     (seq
                                      (ap ("deal_id" joined_deal-0-0_flat) %DealInfo_obj_map)
                                      (ap ("worker_id" joined_deal-0-0.$.worker_id) %DealInfo_obj_map)
                                     )
                                     (canon %init_peer_id% %DealInfo_obj_map  DealInfo_obj)
                                    )
                                   )
                                   (new %DealChangesReq_obj_map
                                    (seq
                                     (seq
                                      (ap ("deal_info" DealInfo_obj) %DealChangesReq_obj_map)
                                      (ap ("left_boundary" state-1_gate.$.[0].[0].left_boundary) %DealChangesReq_obj_map)
                                     )
                                     (canon %init_peer_id% %DealChangesReq_obj_map  DealChangesReq_obj)
                                    )
                                   )
                                  )
                                  (ap DealChangesReq_obj $requests)
                                 )
                                )
                                (seq
                                 (ap :error: -if-error-)
                                 (xor
                                  (match :error:.$.error_code 10002
                                   (null)
                                  )
                                  (fail -if-error-)
                                 )
                                )
                               )
                              )
                             )
                             (next joined_deal-0-0)
                            )
                            (null)
                           )
                          )
                         )
                         (seq
                          (ap :error: -if-error-)
                          (xor
                           (match :error:.$.error_code 10002
                            (null)
                           )
                           (fail -if-error-)
                          )
                         )
                        )
                       )
                       (canon %init_peer_id% $requests  #-requests-fix-0)
                      )
                      (ap #-requests-fix-0 -requests-flat-0)
                     )
                    )
                    (ap -requests-flat-0 -requests-flat-0_to_functor)
                   )
                   (ap -requests-flat-0_to_functor.length -requests-flat-0_length)
                  )
                  (new -if-error-
                   (xor
                    (mismatch -requests-flat-0_length 0
                     (seq
                      (seq
                       (seq
                        (seq
                         (seq
                          (new $array-inline-71
                           (seq
                            (seq
                             (seq
                              (seq
                               (seq
                                (ap -requests-flat-0 -requests-flat-0_to_functor-0)
                                (ap -requests-flat-0_to_functor-0.length -requests-flat-0_length-0)
                               )
                               (ap "try to find updates for" $array-inline-71)
                              )
                              (ap -requests-flat-0_length-0 $array-inline-71)
                             )
                             (ap "deals" $array-inline-71)
                            )
                            (canon %init_peer_id% $array-inline-71  #array-inline-71-0)
                           )
                          )
                          (par
                           (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-71-0])
                           (null)
                          )
                         )
                         (call %init_peer_id% ("json" "stringify") [#array-inline-71-0] ret-244)
                        )
                        (call %init_peer_id% (-spell_id-arg- "store_log") [ret-244] ret-245)
                       )
                       (call %init_peer_id% ("chain_connector" "poll_deal_changes") [-chain-arg-.$.api_endpoint -requests-flat-0] ret-246)
                      )
                      (new -if-else-error-
                       (new -else-error-
                        (new -if-error-
                         (xor
                          (match ret-246.$.success true
                           (ap ret-246.$.changes $changes-0)
                          )
                          (seq
                           (ap :error: -if-error-)
                           (xor
                            (match :error:.$.error_code 10001
                             (seq
                              (seq
                               (seq
                                (new $array-inline-72
                                 (seq
                                  (seq
                                   (ap "error retrieving deal changes from chain" $array-inline-72)
                                   (ap ret-246.$.error.[0] $array-inline-72)
                                  )
                                  (canon %init_peer_id% $array-inline-72  #array-inline-72-0)
                                 )
                                )
                                (par
                                 (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-72-0])
                                 (null)
                                )
                               )
                               (call %init_peer_id% ("json" "stringify") [#array-inline-72-0] ret-247)
                              )
                              (call %init_peer_id% (-spell_id-arg- "store_log") [ret-247] ret-248)
                             )
                            )
                            (seq
                             (seq
                              (ap :error: -else-error-)
                              (xor
                               (match :error:.$.error_code 10001
                                (ap -if-error- -if-else-error-)
                               )
                               (ap -else-error- -if-else-error-)
                              )
                             )
                             (fail -if-else-error-)
                            )
                           )
                          )
                         )
                        )
                       )
                      )
                     )
                    )
                    (seq
                     (ap :error: -if-error-)
                     (xor
                      (match :error:.$.error_code 10002
                       (null)
                      )
                      (fail -if-error-)
                     )
                    )
                   )
                  )
                 )
                 (canon %init_peer_id% $changes-0  #-changes-fix-0)
                )
                (ap #-changes-fix-0 -changes-flat-0)
               )
              )
              (new -if-error-
               (xor
                (mismatch -changes-flat-0 []
                 (seq
                  (fold -changes-flat-0.$.[0] change-0
                   (seq
                    (seq
                     (seq
                      (ap change-0.$.deal_info change-0_flat)
                      (ap change-0_flat.$.deal_id change-0_flat_flat)
                     )
                     (new -if-else-error-
                      (new -else-error-
                       (new -if-error-
                        (xor
                         (match change-0.$.success true
                          (new -if-error-
                           (xor
                            (mismatch change-0.$.log []
                             (seq
                              (seq
                               (seq
                                (seq
                                 (seq
                                  (seq
                                   (seq
                                    (seq
                                     (seq
                                      (seq
                                       (seq
                                        (new $array-inline-73
                                         (seq
                                          (seq
                                           (ap "found a deal changed log on block" $array-inline-73)
                                           (ap change-0.$.log.[0].block_number $array-inline-73)
                                          )
                                          (canon %init_peer_id% $array-inline-73  #array-inline-73-0)
                                         )
                                        )
                                        (call %init_peer_id% ("op" "concat_strings") ["decider deal_id=" change-0_flat_flat ": "] ret-249)
                                       )
                                       (par
                                        (call %init_peer_id% ("run-console" "print") [ret-249 #array-inline-73-0])
                                        (null)
                                       )
                                      )
                                      (call %init_peer_id% ("json" "stringify") [#array-inline-73-0] ret-250)
                                     )
                                     (call %init_peer_id% ("op" "concat_strings") [ret-249 ret-250] ret-251)
                                    )
                                    (call %init_peer_id% (-spell_id-arg- "store_log") [ret-251] ret-252)
                                   )
                                   (xor
                                    (seq
                                     (seq
                                      (seq
                                       (seq
                                        (seq
                                         (seq
                                          (seq
                                           (seq
                                            (seq
                                             (ap change-0.$.deal_info.deal_id change-0_flat-0)
                                             (ap change-0.$.deal_info.worker_id change-0_flat-1)
                                            )
                                            (call %init_peer_id% ("json" "stringify") [change-0.$.log.[0].info.app_cid] ret-253)
                                           )
                                           (new $array-inline-74
                                            (seq
                                             (seq
                                              (seq
                                               (ap "sending the latest update to the worker" $array-inline-74)
                                               (ap change-0_flat-1 $array-inline-74)
                                              )
                                              (ap ret-253 $array-inline-74)
                                             )
                                             (canon %init_peer_id% $array-inline-74  #array-inline-74-0)
                                            )
                                           )
                                          )
                                          (call %init_peer_id% ("op" "concat_strings") ["decider deal_id=" change-0_flat-0 ": "] ret-254)
                                         )
                                         (par
                                          (call %init_peer_id% ("run-console" "print") [ret-254 #array-inline-74-0])
                                          (null)
                                         )
                                        )
                                        (call %init_peer_id% ("json" "stringify") [#array-inline-74-0] ret-255)
                                       )
                                       (call %init_peer_id% ("op" "concat_strings") [ret-254 ret-255] ret-256)
                                      )
                                      (call %init_peer_id% (-spell_id-arg- "store_log") [ret-256] ret-257)
                                     )
                                     (xor
                                      (seq
                                       (seq
                                        (seq
                                         (new $-ephemeral-stream-
                                          (new #-ephemeral-canon-
                                           (canon -relay- $-ephemeral-stream-  #-ephemeral-canon-)
                                          )
                                         )
                                         (call change-0_flat-1 ("worker-spell" "set_string") ["worker_def_cid" ret-253] ret-258)
                                        )
                                        (call change-0_flat-1 ("worker" "is_active") [change-0_flat-0] ret-259)
                                       )
                                       (new -if-error-
                                        (xor
                                         (seq
                                          (match ret-259 true
                                           (call change-0_flat-1 ("spell" "update_trigger_config") ["worker-spell" -worker_settings-arg-.$.config])
                                          )
                                          (new $-ephemeral-stream-
                                           (new #-ephemeral-canon-
                                            (canon -relay- $-ephemeral-stream-  #-ephemeral-canon-)
                                           )
                                          )
                                         )
                                         (seq
                                          (seq
                                           (ap :error: -if-error-)
                                           (xor
                                            (seq
                                             (match :error:.$.error_code 10001
                                              (null)
                                             )
                                             (new $-ephemeral-stream-
                                              (new #-ephemeral-canon-
                                               (canon -relay- $-ephemeral-stream-  #-ephemeral-canon-)
                                              )
                                             )
                                            )
                                            (fail -if-error-)
                                           )
                                          )
                                          (new $-ephemeral-stream-
                                           (new #-ephemeral-canon-
                                            (canon -relay- $-ephemeral-stream-  #-ephemeral-canon-)
                                           )
                                          )
                                         )
                                        )
                                       )
                                      )
                                      (seq
                                       (seq
                                        (new $-ephemeral-stream-
                                         (new #-ephemeral-canon-
                                          (canon -relay- $-ephemeral-stream-  #-ephemeral-canon-)
                                         )
                                        )
                                        (new $-ephemeral-stream-
                                         (new #-ephemeral-canon-
                                          (canon %init_peer_id% $-ephemeral-stream-  #-ephemeral-canon-)
                                         )
                                        )
                                       )
                                       (fail :error:)
                                      )
                                     )
                                    )
                                    (seq
                                     (seq
                                      (seq
                                       (seq
                                        (seq
                                         (new $array-inline-75
                                          (seq
                                           (seq
                                            (ap "can't update worker:" $array-inline-75)
                                            (ap :error: $array-inline-75)
                                           )
                                           (canon %init_peer_id% $array-inline-75  #array-inline-75-0)
                                          )
                                         )
                                         (call %init_peer_id% ("op" "concat_strings") ["decider deal_id=" change-0_flat_flat ": "] ret-260)
                                        )
                                        (par
                                         (call %init_peer_id% ("run-console" "print") [ret-260 #array-inline-75-0])
                                         (null)
                                        )
                                       )
                                       (call %init_peer_id% ("json" "stringify") [#array-inline-75-0] ret-261)
                                      )
                                      (call %init_peer_id% ("op" "concat_strings") [ret-260 ret-261] ret-262)
                                     )
                                     (call %init_peer_id% (-spell_id-arg- "store_log") [ret-262] ret-263)
                                    )
                                   )
                                  )
                                  (ap change-0.$.log change-0_flat-2)
                                 )
                                 (ap change-0_flat-2.$.[0].block_number change-0_flat-2_flat)
                                )
                                (new %DealState_obj-1_map
                                 (seq
                                  (ap ("left_boundary" change-0_flat-2_flat) %DealState_obj-1_map)
                                  (canon %init_peer_id% %DealState_obj-1_map  DealState_obj-1)
                                 )
                                )
                               )
                               (call %init_peer_id% ("json" "stringify") [DealState_obj-1] ret-264)
                              )
                              (call %init_peer_id% (-spell_id-arg- "set_string") [change-0_flat_flat ret-264] ret-265)
                             )
                            )
                            (seq
                             (ap :error: -if-error-)
                             (xor
                              (match :error:.$.error_code 10002
                               (null)
                              )
                              (fail -if-error-)
                             )
                            )
                           )
                          )
                         )
                         (seq
                          (ap :error: -if-error-)
                          (xor
                           (match :error:.$.error_code 10001
                            (seq
                             (seq
                              (seq
                               (seq
                                (seq
                                 (new $array-inline-76
                                  (seq
                                   (seq
                                    (ap "error retrieving deal change" $array-inline-76)
                                    (ap change-0.$.error $array-inline-76)
                                   )
                                   (canon %init_peer_id% $array-inline-76  #array-inline-76-0)
                                  )
                                 )
                                 (call %init_peer_id% ("op" "concat_strings") ["decider deal_id=" change-0_flat_flat ": "] ret-266)
                                )
                                (par
                                 (call %init_peer_id% ("run-console" "print") [ret-266 #array-inline-76-0])
                                 (null)
                                )
                               )
                               (call %init_peer_id% ("json" "stringify") [#array-inline-76-0] ret-267)
                              )
                              (call %init_peer_id% ("op" "concat_strings") [ret-266 ret-267] ret-268)
                             )
                             (call %init_peer_id% (-spell_id-arg- "store_log") [ret-268] ret-269)
                            )
                           )
                           (seq
                            (seq
                             (ap :error: -else-error-)
                             (xor
                              (match :error:.$.error_code 10001
                               (ap -if-error- -if-else-error-)
                              )
                              (ap -else-error- -if-else-error-)
                             )
                            )
                            (fail -if-else-error-)
                           )
                          )
                         )
                        )
                       )
                      )
                     )
                    )
                    (next change-0)
                   )
                   (null)
                  )
                  (par
                   (fold -changes-flat-0.$.[0] change-1
                    (par
                     (new -if-error-
                      (xor
                       (match change-1.$.success true
                        (seq
                         (seq
                          (seq
                           (seq
                            (seq
                             (seq
                              (ap change-1.$.deal_info change-1_flat)
                              (ap change-1_flat.$.deal_id change-1_flat_flat)
                             )
                             (call %init_peer_id% ("chain_connector" "hex_min") [change-1.$.right_boundary -latest-flat-0.$.[0]] ret-270)
                            )
                            (xor
                             (match ret-270.$.success true
                              (ap false not-17)
                             )
                             (ap true not-17)
                            )
                           )
                           (new -if-error-
                            (xor
                             (match not-17 true
                              (seq
                               (seq
                                (seq
                                 (new $array-inline-77
                                  (seq
                                   (seq
                                    (seq
                                     (ap "hex_min failed" $array-inline-77)
                                     (ap change-1.$.right_boundary $array-inline-77)
                                    )
                                    (ap -latest-flat-0.$.[0] $array-inline-77)
                                   )
                                   (canon %init_peer_id% $array-inline-77  #array-inline-77-0)
                                  )
                                 )
                                 (par
                                  (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-77-0])
                                  (null)
                                 )
                                )
                                (call %init_peer_id% ("json" "stringify") [#array-inline-77-0] ret-271)
                               )
                               (call %init_peer_id% (-spell_id-arg- "store_log") [ret-271] ret-272)
                              )
                             )
                             (seq
                              (ap :error: -if-error-)
                              (xor
                               (match :error:.$.error_code 10001
                                (null)
                               )
                               (fail -if-error-)
                              )
                             )
                            )
                           )
                          )
                          (ap ret-270.$.hex ret-270_flat)
                         )
                         (new -if-error-
                          (xor
                           (mismatch ret-270_flat []
                            (seq
                             (seq
                              (seq
                               (seq
                                (call %init_peer_id% ("chain_connector" "hex_add") [ret-270_flat.$.[0] 1] ret-273)
                                (xor
                                 (match ret-273.$.success true
                                  (ap false not-18)
                                 )
                                 (ap true not-18)
                                )
                               )
                               (new -if-error-
                                (xor
                                 (match not-18 true
                                  (seq
                                   (seq
                                    (seq
                                     (new $array-inline-78
                                      (seq
                                       (seq
                                        (seq
                                         (ap "hex_add failed" $array-inline-78)
                                         (ap ret-270_flat.$.[0] $array-inline-78)
                                        )
                                        (ap 1 $array-inline-78)
                                       )
                                       (canon %init_peer_id% $array-inline-78  #array-inline-78-0)
                                      )
                                     )
                                     (par
                                      (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-78-0])
                                      (null)
                                     )
                                    )
                                    (call %init_peer_id% ("json" "stringify") [#array-inline-78-0] ret-274)
                                   )
                                   (call %init_peer_id% (-spell_id-arg- "store_log") [ret-274] ret-275)
                                  )
                                 )
                                 (seq
                                  (ap :error: -if-error-)
                                  (xor
                                   (match :error:.$.error_code 10001
                                    (null)
                                   )
                                   (fail -if-error-)
                                  )
                                 )
                                )
                               )
                              )
                              (ap ret-273.$.hex ret-273_flat)
                             )
                             (new -if-error-
                              (xor
                               (mismatch ret-273_flat []
                                (seq
                                 (seq
                                  (new %DealState_obj-2_map
                                   (seq
                                    (ap ("left_boundary" ret-273_flat.$.[0]) %DealState_obj-2_map)
                                    (canon %init_peer_id% %DealState_obj-2_map  DealState_obj-2)
                                   )
                                  )
                                  (call %init_peer_id% ("json" "stringify") [DealState_obj-2] ret-276)
                                 )
                                 (call %init_peer_id% (-spell_id-arg- "set_string") [change-1_flat_flat ret-276] ret-277)
                                )
                               )
                               (seq
                                (ap :error: -if-error-)
                                (xor
                                 (match :error:.$.error_code 10002
                                  (null)
                                 )
                                 (fail -if-error-)
                                )
                               )
                              )
                             )
                            )
                           )
                           (seq
                            (ap :error: -if-error-)
                            (xor
                             (match :error:.$.error_code 10002
                              (null)
                             )
                             (fail -if-error-)
                            )
                           )
                          )
                         )
                        )
                       )
                       (seq
                        (ap :error: -if-error-)
                        (xor
                         (match :error:.$.error_code 10001
                          (null)
                         )
                         (fail -if-error-)
                        )
                       )
                      )
                     )
                     (next change-1)
                    )
                    (never)
                   )
                   (null)
                  )
                 )
                )
                (seq
                 (ap :error: -if-error-)
                 (xor
                  (match :error:.$.error_code 10002
                   (null)
                  )
                  (fail -if-error-)
                 )
                )
               )
              )
             )
             (null)
            )
           )
           (xor
            (new $deal_ids-1
             (seq
              (seq
               (fold -joined_deals-flat-0 joined_deal-1-0
                (seq
                 (ap joined_deal-1-0.$.deal_id $deal_ids-1)
                 (next joined_deal-1-0)
                )
                (null)
               )
               (canon %init_peer_id% $deal_ids-1  #deal_ids-1_canon)
              )
              (new -if-error-
               (xor
                (mismatch #deal_ids-1_canon []
                 (seq
                  (seq
                   (seq
                    (canon %init_peer_id% $deal_ids-1  #deal_ids-1_canon-0)
                    (call %init_peer_id% ("chain_connector" "get_status_batch") [-chain-arg-.$.api_endpoint #deal_ids-1_canon-0] ret-278)
                   )
                   (xor
                    (match ret-278.$.success true
                     (ap false not-19)
                    )
                    (ap true not-19)
                   )
                  )
                  (new -if-else-error-
                   (new -else-error-
                    (new -if-error-
                     (xor
                      (match not-19 true
                       (seq
                        (seq
                         (seq
                          (new $array-inline-79
                           (seq
                            (seq
                             (ap "couldn't obtain deal statuses, error: " $array-inline-79)
                             (ap ret-278.$.error.[0] $array-inline-79)
                            )
                            (canon %init_peer_id% $array-inline-79  #array-inline-79-0)
                           )
                          )
                          (par
                           (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-79-0])
                           (null)
                          )
                         )
                         (call %init_peer_id% ("json" "stringify") [#array-inline-79-0] ret-279)
                        )
                        (call %init_peer_id% (-spell_id-arg- "store_log") [ret-279] ret-280)
                       )
                      )
                      (seq
                       (ap :error: -if-error-)
                       (xor
                        (match :error:.$.error_code 10001
                         (seq
                          (seq
                           (seq
                            (seq
                             (new $array-inline-80
                              (seq
                               (seq
                                (seq
                                 (seq
                                  (seq
                                   (seq
                                    (ap ret-278.$.statuses ret-278_flat)
                                    (ap ret-278_flat ret-278_flat_to_functor)
                                   )
                                   (ap ret-278_flat_to_functor.length ret-278_flat_length)
                                  )
                                  (ap "found statuses for" $array-inline-80)
                                 )
                                 (ap ret-278_flat_length $array-inline-80)
                                )
                                (ap "deals" $array-inline-80)
                               )
                               (canon %init_peer_id% $array-inline-80  #array-inline-80-0)
                              )
                             )
                             (par
                              (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-80-0])
                              (null)
                             )
                            )
                            (call %init_peer_id% ("json" "stringify") [#array-inline-80-0] ret-281)
                           )
                           (call %init_peer_id% (-spell_id-arg- "store_log") [ret-281] ret-282)
                          )
                          (fold ret-278.$.statuses deal_status-1-0
                           (seq
                            (seq
                             (xor
                              (match deal_status-1-0.$.success true
                               (ap false not-20)
                              )
                              (ap true not-20)
                             )
                             (new -if-else-error-
                              (new -else-error-
                               (new -if-error-
                                (xor
                                 (match not-20 true
                                  (seq
                                   (seq
                                    (seq
                                     (new $array-inline-81
                                      (seq
                                       (seq
                                        (ap "couldn't obtain deal status, error:" $array-inline-81)
                                        (ap deal_status-1-0.$.error.[0] $array-inline-81)
                                       )
                                       (canon %init_peer_id% $array-inline-81  #array-inline-81-0)
                                      )
                                     )
                                     (par
                                      (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-81-0])
                                      (null)
                                     )
                                    )
                                    (call %init_peer_id% ("json" "stringify") [#array-inline-81-0] ret-283)
                                   )
                                   (call %init_peer_id% (-spell_id-arg- "store_log") [ret-283] ret-284)
                                  )
                                 )
                                 (seq
                                  (ap :error: -if-error-)
                                  (xor
                                   (match :error:.$.error_code 10001
                                    (new -if-else-error-
                                     (new -else-error-
                                      (new -if-error-
                                       (xor
                                        (match deal_status-1-0.$.status "ACTIVE"
                                         (xor
                                          (seq
                                           (seq
                                            (call %init_peer_id% ("worker" "is_active") [deal_status-1-0.$.deal_id] ret-285)
                                            (xor
                                             (match ret-285 true
                                              (ap false not-21)
                                             )
                                             (ap true not-21)
                                            )
                                           )
                                           (new -if-error-
                                            (xor
                                             (match not-21 true
                                              (seq
                                               (seq
                                                (seq
                                                 (seq
                                                  (seq
                                                   (call %init_peer_id% ("op" "concat_strings") ["decider deal_id=" deal_status-1-0.$.deal_id ": "] ret-286)
                                                   (par
                                                    (call %init_peer_id% ("run-console" "print") [ret-286 "activating worker"])
                                                    (null)
                                                   )
                                                  )
                                                  (call %init_peer_id% ("json" "stringify") ["activating worker"] ret-287)
                                                 )
                                                 (call %init_peer_id% ("op" "concat_strings") [ret-286 ret-287] ret-288)
                                                )
                                                (call %init_peer_id% (-spell_id-arg- "store_log") [ret-288] ret-289)
                                               )
                                               (call %init_peer_id% ("worker" "activate") [deal_status-1-0.$.deal_id])
                                              )
                                             )
                                             (seq
                                              (ap :error: -if-error-)
                                              (xor
                                               (match :error:.$.error_code 10001
                                                (null)
                                               )
                                               (fail -if-error-)
                                              )
                                             )
                                            )
                                           )
                                          )
                                          (seq
                                           (seq
                                            (seq
                                             (seq
                                              (seq
                                               (new $array-inline-82
                                                (seq
                                                 (seq
                                                  (ap "couldn't activate a worker" $array-inline-82)
                                                  (ap :error: $array-inline-82)
                                                 )
                                                 (canon %init_peer_id% $array-inline-82  #array-inline-82-0)
                                                )
                                               )
                                               (call %init_peer_id% ("op" "concat_strings") ["decider deal_id=" deal_status-1-0.$.deal_id ": "] ret-290)
                                              )
                                              (par
                                               (call %init_peer_id% ("run-console" "print") [ret-290 #array-inline-82-0])
                                               (null)
                                              )
                                             )
                                             (call %init_peer_id% ("json" "stringify") [#array-inline-82-0] ret-291)
                                            )
                                            (call %init_peer_id% ("op" "concat_strings") [ret-290 ret-291] ret-292)
                                           )
                                           (call %init_peer_id% (-spell_id-arg- "store_log") [ret-292] ret-293)
                                          )
                                         )
                                        )
                                        (seq
                                         (ap :error: -if-error-)
                                         (xor
                                          (match :error:.$.error_code 10001
                                           (new -if-else-error-
                                            (new -else-error-
                                             (new -if-error-
                                              (xor
                                               (match deal_status-1-0.$.status "INACTIVE"
                                                (xor
                                                 (seq
                                                  (call %init_peer_id% ("worker" "is_active") [deal_status-1-0.$.deal_id] ret-294)
                                                  (new -if-error-
                                                   (xor
                                                    (match ret-294 true
                                                     (seq
                                                      (seq
                                                       (seq
                                                        (seq
                                                         (seq
                                                          (call %init_peer_id% ("op" "concat_strings") ["decider deal_id=" deal_status-1-0.$.deal_id ": "] ret-295)
                                                          (par
                                                           (call %init_peer_id% ("run-console" "print") [ret-295 "deactivating worker"])
                                                           (null)
                                                          )
                                                         )
                                                         (call %init_peer_id% ("json" "stringify") ["deactivating worker"] ret-296)
                                                        )
                                                        (call %init_peer_id% ("op" "concat_strings") [ret-295 ret-296] ret-297)
                                                       )
                                                       (call %init_peer_id% (-spell_id-arg- "store_log") [ret-297] ret-298)
                                                      )
                                                      (call %init_peer_id% ("worker" "deactivate") [deal_status-1-0.$.deal_id])
                                                     )
                                                    )
                                                    (seq
                                                     (ap :error: -if-error-)
                                                     (xor
                                                      (match :error:.$.error_code 10001
                                                       (null)
                                                      )
                                                      (fail -if-error-)
                                                     )
                                                    )
                                                   )
                                                  )
                                                 )
                                                 (seq
                                                  (seq
                                                   (seq
                                                    (seq
                                                     (seq
                                                      (new $array-inline-83
                                                       (seq
                                                        (seq
                                                         (ap "couldn't deactivate a worker" $array-inline-83)
                                                         (ap :error: $array-inline-83)
                                                        )
                                                        (canon %init_peer_id% $array-inline-83  #array-inline-83-0)
                                                       )
                                                      )
                                                      (call %init_peer_id% ("op" "concat_strings") ["decider deal_id=" deal_status-1-0.$.deal_id ": "] ret-299)
                                                     )
                                                     (par
                                                      (call %init_peer_id% ("run-console" "print") [ret-299 #array-inline-83-0])
                                                      (null)
                                                     )
                                                    )
                                                    (call %init_peer_id% ("json" "stringify") [#array-inline-83-0] ret-300)
                                                   )
                                                   (call %init_peer_id% ("op" "concat_strings") [ret-299 ret-300] ret-301)
                                                  )
                                                  (call %init_peer_id% (-spell_id-arg- "store_log") [ret-301] ret-302)
                                                 )
                                                )
                                               )
                                               (seq
                                                (ap :error: -if-error-)
                                                (xor
                                                 (match :error:.$.error_code 10001
                                                  (new -if-else-error-
                                                   (new -else-error-
                                                    (new -if-error-
                                                     (xor
                                                      (match deal_status-1-0.$.status "ENDED"
                                                       (seq
                                                        (call %init_peer_id% ("worker" "get_worker_id") [deal_status-1-0.$.deal_id] ret-303)
                                                        (new -if-else-error-
                                                         (new -else-error-
                                                          (new -if-error-
                                                           (xor
                                                            (mismatch ret-303 []
                                                             (seq
                                                              (seq
                                                               (seq
                                                                (seq
                                                                 (seq
                                                                  (seq
                                                                   (call %init_peer_id% ("op" "concat_strings") ["decider deal_id=" deal_status-1-0.$.deal_id ": "] ret-304)
                                                                   (par
                                                                    (call %init_peer_id% ("run-console" "print") [ret-304 "removing the worker and the deal state from kv"])
                                                                    (null)
                                                                   )
                                                                  )
                                                                  (call %init_peer_id% ("json" "stringify") ["removing the worker and the deal state from kv"] ret-305)
                                                                 )
                                                                 (call %init_peer_id% ("op" "concat_strings") [ret-304 ret-305] ret-306)
                                                                )
                                                                (call %init_peer_id% (-spell_id-arg- "store_log") [ret-306] ret-307)
                                                               )
                                                               (xor
                                                                (call %init_peer_id% ("worker" "remove") [ret-303.$.[0]])
                                                                (seq
                                                                 (seq
                                                                  (seq
                                                                   (seq
                                                                    (seq
                                                                     (new $array-inline-84
                                                                      (seq
                                                                       (seq
                                                                        (seq
                                                                         (ap "couldn't remove a worker" $array-inline-84)
                                                                         (ap ret-303.$.[0] $array-inline-84)
                                                                        )
                                                                        (ap :error: $array-inline-84)
                                                                       )
                                                                       (canon %init_peer_id% $array-inline-84  #array-inline-84-0)
                                                                      )
                                                                     )
                                                                     (call %init_peer_id% ("op" "concat_strings") ["decider deal_id=" deal_status-1-0.$.deal_id ": "] ret-308)
                                                                    )
                                                                    (par
                                                                     (call %init_peer_id% ("run-console" "print") [ret-308 #array-inline-84-0])
                                                                     (null)
                                                                    )
                                                                   )
                                                                   (call %init_peer_id% ("json" "stringify") [#array-inline-84-0] ret-309)
                                                                  )
                                                                  (call %init_peer_id% ("op" "concat_strings") [ret-308 ret-309] ret-310)
                                                                 )
                                                                 (call %init_peer_id% (-spell_id-arg- "store_log") [ret-310] ret-311)
                                                                )
                                                               )
                                                              )
                                                              (xor
                                                               (seq
                                                                (seq
                                                                 (seq
                                                                  (seq
                                                                   (seq
                                                                    (new %JoinedDeal_obj-1_map
                                                                     (seq
                                                                      (seq
                                                                       (ap ("deal_id" deal_status-1-0.$.deal_id) %JoinedDeal_obj-1_map)
                                                                       (ap ("worker_id" ret-303.$.[0]) %JoinedDeal_obj-1_map)
                                                                      )
                                                                      (canon %init_peer_id% %JoinedDeal_obj-1_map  JoinedDeal_obj-1)
                                                                     )
                                                                    )
                                                                    (call %init_peer_id% ("json" "stringify") [JoinedDeal_obj-1] ret-312)
                                                                   )
                                                                   (call %init_peer_id% (-spell_id-arg- "list_remove_string") ["joined_deals" ret-312] ret-313)
                                                                  )
                                                                  (call %init_peer_id% (-spell_id-arg- "remove_key") [deal_status-1-0.$.deal_id] ret-314)
                                                                 )
                                                                 (call %init_peer_id% ("op" "concat_strings") ["removed_state:" deal_status-1-0.$.deal_id] ret-315)
                                                                )
                                                                (call %init_peer_id% (-spell_id-arg- "remove_key") [ret-315] ret-316)
                                                               )
                                                               (seq
                                                                (seq
                                                                 (seq
                                                                  (seq
                                                                   (seq
                                                                    (new $array-inline-85
                                                                     (seq
                                                                      (seq
                                                                       (ap "couldn't remove the deal" $array-inline-85)
                                                                       (ap :error: $array-inline-85)
                                                                      )
                                                                      (canon %init_peer_id% $array-inline-85  #array-inline-85-0)
                                                                     )
                                                                    )
                                                                    (call %init_peer_id% ("op" "concat_strings") ["decider deal_id=" deal_status-1-0.$.deal_id ": "] ret-317)
                                                                   )
                                                                   (par
                                                                    (call %init_peer_id% ("run-console" "print") [ret-317 #array-inline-85-0])
                                                                    (null)
                                                                   )
                                                                  )
                                                                  (call %init_peer_id% ("json" "stringify") [#array-inline-85-0] ret-318)
                                                                 )
                                                                 (call %init_peer_id% ("op" "concat_strings") [ret-317 ret-318] ret-319)
                                                                )
                                                                (call %init_peer_id% (-spell_id-arg- "store_log") [ret-319] ret-320)
                                                               )
                                                              )
                                                             )
                                                            )
                                                            (seq
                                                             (ap :error: -if-error-)
                                                             (xor
                                                              (match :error:.$.error_code 10002
                                                               (seq
                                                                (seq
                                                                 (seq
                                                                  (seq
                                                                   (call %init_peer_id% ("op" "concat_strings") ["decider deal_id=" deal_status-1-0.$.deal_id ": "] ret-321)
                                                                   (par
                                                                    (call %init_peer_id% ("run-console" "print") [ret-321 "deal doesn't have associated worker O.o"])
                                                                    (null)
                                                                   )
                                                                  )
                                                                  (call %init_peer_id% ("json" "stringify") ["deal doesn't have associated worker O.o"] ret-322)
                                                                 )
                                                                 (call %init_peer_id% ("op" "concat_strings") [ret-321 ret-322] ret-323)
                                                                )
                                                                (call %init_peer_id% (-spell_id-arg- "store_log") [ret-323] ret-324)
                                                               )
                                                              )
                                                              (seq
                                                               (seq
                                                                (ap :error: -else-error-)
                                                                (xor
                                                                 (match :error:.$.error_code 10001
                                                                  (ap -if-error- -if-else-error-)
                                                                 )
                                                                 (ap -else-error- -if-else-error-)
                                                                )
                                                               )
                                                               (fail -if-else-error-)
                                                              )
                                                             )
                                                            )
                                                           )
                                                          )
                                                         )
                                                        )
                                                       )
                                                      )
                                                      (seq
                                                       (ap :error: -if-error-)
                                                       (xor
                                                        (match :error:.$.error_code 10001
                                                         (seq
                                                          (seq
                                                           (seq
                                                            (new $array-inline-86
                                                             (seq
                                                              (seq
                                                               (ap "unsupported deal status: " $array-inline-86)
                                                               (ap deal_status-1-0 $array-inline-86)
                                                              )
                                                              (canon %init_peer_id% $array-inline-86  #array-inline-86-0)
                                                             )
                                                            )
                                                            (par
                                                             (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-86-0])
                                                             (null)
                                                            )
                                                           )
                                                           (call %init_peer_id% ("json" "stringify") [#array-inline-86-0] ret-325)
                                                          )
                                                          (call %init_peer_id% (-spell_id-arg- "store_log") [ret-325] ret-326)
                                                         )
                                                        )
                                                        (seq
                                                         (seq
                                                          (ap :error: -else-error-)
                                                          (xor
                                                           (match :error:.$.error_code 10001
                                                            (ap -if-error- -if-else-error-)
                                                           )
                                                           (ap -else-error- -if-else-error-)
                                                          )
                                                         )
                                                         (fail -if-else-error-)
                                                        )
                                                       )
                                                      )
                                                     )
                                                    )
                                                   )
                                                  )
                                                 )
                                                 (seq
                                                  (seq
                                                   (ap :error: -else-error-)
                                                   (xor
                                                    (match :error:.$.error_code 10001
                                                     (ap -if-error- -if-else-error-)
                                                    )
                                                    (ap -else-error- -if-else-error-)
                                                   )
                                                  )
                                                  (fail -if-else-error-)
                                                 )
                                                )
                                               )
                                              )
                                             )
                                            )
                                           )
                                          )
                                          (seq
                                           (seq
                                            (ap :error: -else-error-)
                                            (xor
                                             (match :error:.$.error_code 10001
                                              (ap -if-error- -if-else-error-)
                                             )
                                             (ap -else-error- -if-else-error-)
                                            )
                                           )
                                           (fail -if-else-error-)
                                          )
                                         )
                                        )
                                       )
                                      )
                                     )
                                    )
                                   )
                                   (seq
                                    (seq
                                     (ap :error: -else-error-)
                                     (xor
                                      (match :error:.$.error_code 10001
                                       (ap -if-error- -if-else-error-)
                                      )
                                      (ap -else-error- -if-else-error-)
                                     )
                                    )
                                    (fail -if-else-error-)
                                   )
                                  )
                                 )
                                )
                               )
                              )
                             )
                            )
                            (next deal_status-1-0)
                           )
                           (null)
                          )
                         )
                        )
                        (seq
                         (seq
                          (ap :error: -else-error-)
                          (xor
                           (match :error:.$.error_code 10001
                            (ap -if-error- -if-else-error-)
                           )
                           (ap -else-error- -if-else-error-)
                          )
                         )
                         (fail -if-else-error-)
                        )
                       )
                      )
                     )
                    )
                   )
                  )
                 )
                )
                (seq
                 (ap :error: -if-error-)
                 (xor
                  (match :error:.$.error_code 10002
                   (null)
                  )
                  (fail -if-error-)
                 )
                )
               )
              )
             )
            )
            (null)
           )
          )
          (xor
           (seq
            (seq
             (ap -joined_deals-flat-0 -joined_deals-flat-0_to_functor)
             (ap -joined_deals-flat-0_to_functor.length -joined_deals-flat-0_length)
            )
            (new -if-error-
             (xor
              (mismatch -joined_deals-flat-0_length 0
               (new $reqs
                (seq
                 (seq
                  (fold -joined_deals-flat-0 joined_deal-3-0
                   (seq
                    (seq
                     (seq
                      (ap joined_deal-3-0.$.deal_id joined_deal-3-0_flat)
                      (new $state-6
                       (seq
                        (seq
                         (seq
                          (seq
                           (call %init_peer_id% ("op" "concat_strings") ["removed_state:" joined_deal-3-0_flat] ret-327)
                           (call %init_peer_id% (-spell_id-arg- "get_string") [ret-327] ret-328)
                          )
                          (xor
                           (match ret-328.$.success false
                            (ap ret-328.$.success and-0)
                           )
                           (seq
                            (xor
                             (match ret-328.$.absent true
                              (ap false not-22)
                             )
                             (ap true not-22)
                            )
                            (ap not-22 and-0)
                           )
                          )
                         )
                         (new -if-else-error-
                          (new -else-error-
                           (new -if-error-
                            (xor
                             (match and-0 true
                              (seq
                               (new $state-7
                                (seq
                                 (seq
                                  (xor
                                   (seq
                                    (call %init_peer_id% ("json" "parse") [ret-328.$.value] ret-329)
                                    (ap ret-329 $state-7)
                                   )
                                   (seq
                                    (seq
                                     (seq
                                      (seq
                                       (seq
                                        (new $array-inline-87
                                         (seq
                                          (seq
                                           (ap "failed to parse deal state from json" $array-inline-87)
                                           (ap :error: $array-inline-87)
                                          )
                                          (canon %init_peer_id% $array-inline-87  #array-inline-87-0)
                                         )
                                        )
                                        (call %init_peer_id% ("op" "concat_strings") ["decider deal_id=" joined_deal-3-0_flat ": "] ret-330)
                                       )
                                       (par
                                        (call %init_peer_id% ("run-console" "print") [ret-330 #array-inline-87-0])
                                        (null)
                                       )
                                      )
                                      (call %init_peer_id% ("json" "stringify") [#array-inline-87-0] ret-331)
                                     )
                                     (call %init_peer_id% ("op" "concat_strings") [ret-330 ret-331] ret-332)
                                    )
                                    (call %init_peer_id% (-spell_id-arg- "store_log") [ret-332] ret-333)
                                   )
                                  )
                                  (canon %init_peer_id% $state-7  #-state-fix-0-0)
                                 )
                                 (ap #-state-fix-0-0 -state-flat-0-0)
                                )
                               )
                               (ap -state-flat-0-0 $state-6)
                              )
                             )
                             (seq
                              (ap :error: -if-error-)
                              (xor
                               (match :error:.$.error_code 10001
                                (seq
                                 (seq
                                  (seq
                                   (seq
                                    (seq
                                     (seq
                                      (new $array-inline-88
                                       (seq
                                        (seq
                                         (ap "deal removed state not found:" $array-inline-88)
                                         (ap ret-328.$.error $array-inline-88)
                                        )
                                        (canon %init_peer_id% $array-inline-88  #array-inline-88-0)
                                       )
                                      )
                                      (call %init_peer_id% ("op" "concat_strings") ["decider deal_id=" joined_deal-3-0_flat ": "] ret-334)
                                     )
                                     (par
                                      (call %init_peer_id% ("run-console" "print") [ret-334 #array-inline-88-0])
                                      (null)
                                     )
                                    )
                                    (call %init_peer_id% ("json" "stringify") [#array-inline-88-0] ret-335)
                                   )
                                   (call %init_peer_id% ("op" "concat_strings") [ret-334 ret-335] ret-336)
                                  )
                                  (call %init_peer_id% (-spell_id-arg- "store_log") [ret-336] ret-337)
                                 )
                                 (ap [] $state-6)
                                )
                               )
                               (seq
                                (seq
                                 (ap :error: -else-error-)
                                 (xor
                                  (match :error:.$.error_code 10001
                                   (ap -if-error- -if-else-error-)
                                  )
                                  (ap -else-error- -if-else-error-)
                                 )
                                )
                                (fail -if-else-error-)
                               )
                              )
                             )
                            )
                           )
                          )
                         )
                        )
                        (new $state-6_test
                         (seq
                          (seq
                           (fold $state-6 state-6_fold_var
                            (seq
                             (seq
                              (ap state-6_fold_var $state-6_test)
                              (canon %init_peer_id% $state-6_test  #state-6_iter_canon)
                             )
                             (xor
                              (match #state-6_iter_canon.length 1
                               (null)
                              )
                              (next state-6_fold_var)
                             )
                            )
                            (never)
                           )
                           (canon %init_peer_id% $state-6_test  #state-6_result_canon)
                          )
                          (ap #state-6_result_canon state-6_gate)
                         )
                        )
                       )
                      )
                     )
                     (new -if-error-
                      (xor
                       (mismatch state-6_gate.$.[0] []
                        (seq
                         (new %DealPeerRemovedReq_obj_map
                          (seq
                           (seq
                            (ap ("deal_id" joined_deal-3-0_flat) %DealPeerRemovedReq_obj_map)
                            (ap ("left_boundary" state-6_gate.$.[0].[0].left_boundary) %DealPeerRemovedReq_obj_map)
                           )
                           (canon %init_peer_id% %DealPeerRemovedReq_obj_map  DealPeerRemovedReq_obj)
                          )
                         )
                         (ap DealPeerRemovedReq_obj $reqs)
                        )
                       )
                       (seq
                        (ap :error: -if-error-)
                        (xor
                         (match :error:.$.error_code 10002
                          (null)
                         )
                         (fail -if-error-)
                        )
                       )
                      )
                     )
                    )
                    (next joined_deal-3-0)
                   )
                   (null)
                  )
                  (canon %init_peer_id% $reqs  #reqs_canon)
                 )
                 (new -if-error-
                  (xor
                   (mismatch #reqs_canon []
                    (seq
                     (seq
                      (seq
                       (seq
                        (seq
                         (seq
                          (seq
                           (new $array-inline-89
                            (seq
                             (seq
                              (seq
                               (seq
                                (seq
                                 (canon %init_peer_id% $reqs  #reqs_to_functor)
                                 (ap #reqs_to_functor.length reqs_length)
                                )
                                (ap "find if deals are removed from the provider for" $array-inline-89)
                               )
                               (ap reqs_length $array-inline-89)
                              )
                              (ap "deals" $array-inline-89)
                             )
                             (canon %init_peer_id% $array-inline-89  #array-inline-89-0)
                            )
                           )
                           (par
                            (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-89-0])
                            (null)
                           )
                          )
                          (call %init_peer_id% ("json" "stringify") [#array-inline-89-0] ret-338)
                         )
                         (call %init_peer_id% (-spell_id-arg- "store_log") [ret-338] ret-339)
                        )
                        (canon %init_peer_id% $reqs  #reqs_canon-0)
                       )
                       (call %init_peer_id% ("chain_connector" "poll_deal_peer_removed_batch") [-chain-arg-.$.api_endpoint #reqs_canon-0] ret-340)
                      )
                      (xor
                       (match ret-340.$.success true
                        (ap false not-23)
                       )
                       (ap true not-23)
                      )
                     )
                     (new -if-else-error-
                      (new -else-error-
                       (new -if-error-
                        (xor
                         (match not-23 true
                          (seq
                           (seq
                            (seq
                             (new $array-inline-90
                              (seq
                               (seq
                                (ap "can't find if deals are removed from provider:" $array-inline-90)
                                (ap ret-340.$.error $array-inline-90)
                               )
                               (canon %init_peer_id% $array-inline-90  #array-inline-90-0)
                              )
                             )
                             (par
                              (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-90-0])
                              (null)
                             )
                            )
                            (call %init_peer_id% ("json" "stringify") [#array-inline-90-0] ret-341)
                           )
                           (call %init_peer_id% (-spell_id-arg- "store_log") [ret-341] ret-342)
                          )
                         )
                         (seq
                          (ap :error: -if-error-)
                          (xor
                           (match :error:.$.error_code 10001
                            (fold ret-340.$.result removed-0
                             (seq
                              (seq
                               (xor
                                (match removed-0.$.success true
                                 (ap false not-24)
                                )
                                (ap true not-24)
                               )
                               (new -if-else-error-
                                (new -else-error-
                                 (new -if-error-
                                  (xor
                                   (match not-24 true
                                    (seq
                                     (seq
                                      (seq
                                       (seq
                                        (seq
                                         (new $array-inline-91
                                          (seq
                                           (seq
                                            (ap "can't find if the deal was removed from provider" $array-inline-91)
                                            (ap removed-0.$.error $array-inline-91)
                                           )
                                           (canon %init_peer_id% $array-inline-91  #array-inline-91-0)
                                          )
                                         )
                                         (call %init_peer_id% ("op" "concat_strings") ["decider deal_id=" removed-0.$.deal_id ": "] ret-343)
                                        )
                                        (par
                                         (call %init_peer_id% ("run-console" "print") [ret-343 #array-inline-91-0])
                                         (null)
                                        )
                                       )
                                       (call %init_peer_id% ("json" "stringify") [#array-inline-91-0] ret-344)
                                      )
                                      (call %init_peer_id% ("op" "concat_strings") [ret-343 ret-344] ret-345)
                                     )
                                     (call %init_peer_id% (-spell_id-arg- "store_log") [ret-345] ret-346)
                                    )
                                   )
                                   (seq
                                    (ap :error: -if-error-)
                                    (xor
                                     (match :error:.$.error_code 10001
                                      (new -if-else-error-
                                       (new -else-error-
                                        (new -if-error-
                                         (xor
                                          (match removed-0.$.is_removed true
                                           (seq
                                            (seq
                                             (seq
                                              (seq
                                               (seq
                                                (seq
                                                 (seq
                                                  (new $array-inline-92
                                                   (seq
                                                    (ap "the deal is removed from the provider" $array-inline-92)
                                                    (canon %init_peer_id% $array-inline-92  #array-inline-92-0)
                                                   )
                                                  )
                                                  (call %init_peer_id% ("op" "concat_strings") ["decider deal_id=" removed-0.$.deal_id ": "] ret-347)
                                                 )
                                                 (par
                                                  (call %init_peer_id% ("run-console" "print") [ret-347 #array-inline-92-0])
                                                  (null)
                                                 )
                                                )
                                                (call %init_peer_id% ("json" "stringify") [#array-inline-92-0] ret-348)
                                               )
                                               (call %init_peer_id% ("op" "concat_strings") [ret-347 ret-348] ret-349)
                                              )
                                              (call %init_peer_id% (-spell_id-arg- "store_log") [ret-349] ret-350)
                                             )
                                             (call %init_peer_id% ("worker" "get_worker_id") [removed-0.$.deal_id] ret-351)
                                            )
                                            (new -if-else-error-
                                             (new -else-error-
                                              (new -if-error-
                                               (xor
                                                (mismatch ret-351 []
                                                 (seq
                                                  (seq
                                                   (seq
                                                    (seq
                                                     (seq
                                                      (seq
                                                       (call %init_peer_id% ("op" "concat_strings") ["decider deal_id=" removed-0.$.deal_id ": "] ret-352)
                                                       (par
                                                        (call %init_peer_id% ("run-console" "print") [ret-352 "removing the worker and the deal state from kv"])
                                                        (null)
                                                       )
                                                      )
                                                      (call %init_peer_id% ("json" "stringify") ["removing the worker and the deal state from kv"] ret-353)
                                                     )
                                                     (call %init_peer_id% ("op" "concat_strings") [ret-352 ret-353] ret-354)
                                                    )
                                                    (call %init_peer_id% (-spell_id-arg- "store_log") [ret-354] ret-355)
                                                   )
                                                   (xor
                                                    (call %init_peer_id% ("worker" "remove") [ret-351.$.[0]])
                                                    (seq
                                                     (seq
                                                      (seq
                                                       (seq
                                                        (seq
                                                         (new $array-inline-93
                                                          (seq
                                                           (seq
                                                            (seq
                                                             (ap "couldn't remove a worker" $array-inline-93)
                                                             (ap ret-351.$.[0] $array-inline-93)
                                                            )
                                                            (ap :error: $array-inline-93)
                                                           )
                                                           (canon %init_peer_id% $array-inline-93  #array-inline-93-0)
                                                          )
                                                         )
                                                         (call %init_peer_id% ("op" "concat_strings") ["decider deal_id=" removed-0.$.deal_id ": "] ret-356)
                                                        )
                                                        (par
                                                         (call %init_peer_id% ("run-console" "print") [ret-356 #array-inline-93-0])
                                                         (null)
                                                        )
                                                       )
                                                       (call %init_peer_id% ("json" "stringify") [#array-inline-93-0] ret-357)
                                                      )
                                                      (call %init_peer_id% ("op" "concat_strings") [ret-356 ret-357] ret-358)
                                                     )
                                                     (call %init_peer_id% (-spell_id-arg- "store_log") [ret-358] ret-359)
                                                    )
                                                   )
                                                  )
                                                  (xor
                                                   (seq
                                                    (seq
                                                     (seq
                                                      (seq
                                                       (seq
                                                        (new %JoinedDeal_obj-2_map
                                                         (seq
                                                          (seq
                                                           (ap ("deal_id" removed-0.$.deal_id) %JoinedDeal_obj-2_map)
                                                           (ap ("worker_id" ret-351.$.[0]) %JoinedDeal_obj-2_map)
                                                          )
                                                          (canon %init_peer_id% %JoinedDeal_obj-2_map  JoinedDeal_obj-2)
                                                         )
                                                        )
                                                        (call %init_peer_id% ("json" "stringify") [JoinedDeal_obj-2] ret-360)
                                                       )
                                                       (call %init_peer_id% (-spell_id-arg- "list_remove_string") ["joined_deals" ret-360] ret-361)
                                                      )
                                                      (call %init_peer_id% (-spell_id-arg- "remove_key") [removed-0.$.deal_id] ret-362)
                                                     )
                                                     (call %init_peer_id% ("op" "concat_strings") ["removed_state:" removed-0.$.deal_id] ret-363)
                                                    )
                                                    (call %init_peer_id% (-spell_id-arg- "remove_key") [ret-363] ret-364)
                                                   )
                                                   (seq
                                                    (seq
                                                     (seq
                                                      (seq
                                                       (seq
                                                        (new $array-inline-94
                                                         (seq
                                                          (seq
                                                           (ap "couldn't remove the deal" $array-inline-94)
                                                           (ap :error: $array-inline-94)
                                                          )
                                                          (canon %init_peer_id% $array-inline-94  #array-inline-94-0)
                                                         )
                                                        )
                                                        (call %init_peer_id% ("op" "concat_strings") ["decider deal_id=" removed-0.$.deal_id ": "] ret-365)
                                                       )
                                                       (par
                                                        (call %init_peer_id% ("run-console" "print") [ret-365 #array-inline-94-0])
                                                        (null)
                                                       )
                                                      )
                                                      (call %init_peer_id% ("json" "stringify") [#array-inline-94-0] ret-366)
                                                     )
                                                     (call %init_peer_id% ("op" "concat_strings") [ret-365 ret-366] ret-367)
                                                    )
                                                    (call %init_peer_id% (-spell_id-arg- "store_log") [ret-367] ret-368)
                                                   )
                                                  )
                                                 )
                                                )
                                                (seq
                                                 (ap :error: -if-error-)
                                                 (xor
                                                  (match :error:.$.error_code 10002
                                                   (seq
                                                    (seq
                                                     (seq
                                                      (seq
                                                       (call %init_peer_id% ("op" "concat_strings") ["decider deal_id=" removed-0.$.deal_id ": "] ret-369)
                                                       (par
                                                        (call %init_peer_id% ("run-console" "print") [ret-369 "deal doesn't have associated worker O.o"])
                                                        (null)
                                                       )
                                                      )
                                                      (call %init_peer_id% ("json" "stringify") ["deal doesn't have associated worker O.o"] ret-370)
                                                     )
                                                     (call %init_peer_id% ("op" "concat_strings") [ret-369 ret-370] ret-371)
                                                    )
                                                    (call %init_peer_id% (-spell_id-arg- "store_log") [ret-371] ret-372)
                                                   )
                                                  )
                                                  (seq
                                                   (seq
                                                    (ap :error: -else-error-)
                                                    (xor
                                                     (match :error:.$.error_code 10001
                                                      (ap -if-error- -if-else-error-)
                                                     )
                                                     (ap -else-error- -if-else-error-)
                                                    )
                                                   )
                                                   (fail -if-else-error-)
                                                  )
                                                 )
                                                )
                                               )
                                              )
                                             )
                                            )
                                           )
                                          )
                                          (seq
                                           (ap :error: -if-error-)
                                           (xor
                                            (match :error:.$.error_code 10001
                                             (seq
                                              (seq
                                               (seq
                                                (seq
                                                 (call %init_peer_id% ("chain_connector" "hex_min") [removed-0.$.right_boundary -latest-flat-0.$.[0]] ret-373)
                                                 (xor
                                                  (match ret-373.$.success true
                                                   (ap false not-25)
                                                  )
                                                  (ap true not-25)
                                                 )
                                                )
                                                (new -if-error-
                                                 (xor
                                                  (match not-25 true
                                                   (seq
                                                    (seq
                                                     (seq
                                                      (new $array-inline-95
                                                       (seq
                                                        (seq
                                                         (seq
                                                          (ap "hex_min failed" $array-inline-95)
                                                          (ap removed-0.$.right_boundary $array-inline-95)
                                                         )
                                                         (ap -latest-flat-0.$.[0] $array-inline-95)
                                                        )
                                                        (canon %init_peer_id% $array-inline-95  #array-inline-95-0)
                                                       )
                                                      )
                                                      (par
                                                       (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-95-0])
                                                       (null)
                                                      )
                                                     )
                                                     (call %init_peer_id% ("json" "stringify") [#array-inline-95-0] ret-374)
                                                    )
                                                    (call %init_peer_id% (-spell_id-arg- "store_log") [ret-374] ret-375)
                                                   )
                                                  )
                                                  (seq
                                                   (ap :error: -if-error-)
                                                   (xor
                                                    (match :error:.$.error_code 10001
                                                     (null)
                                                    )
                                                    (fail -if-error-)
                                                   )
                                                  )
                                                 )
                                                )
                                               )
                                               (ap ret-373.$.hex ret-373_flat)
                                              )
                                              (new -if-error-
                                               (xor
                                                (mismatch ret-373_flat []
                                                 (seq
                                                  (seq
                                                   (seq
                                                    (seq
                                                     (call %init_peer_id% ("chain_connector" "hex_add") [ret-373_flat.$.[0] 1] ret-376)
                                                     (xor
                                                      (match ret-376.$.success true
                                                       (ap false not-26)
                                                      )
                                                      (ap true not-26)
                                                     )
                                                    )
                                                    (new -if-error-
                                                     (xor
                                                      (match not-26 true
                                                       (seq
                                                        (seq
                                                         (seq
                                                          (new $array-inline-96
                                                           (seq
                                                            (seq
                                                             (seq
                                                              (ap "hex_add failed" $array-inline-96)
                                                              (ap ret-373_flat.$.[0] $array-inline-96)
                                                             )
                                                             (ap 1 $array-inline-96)
                                                            )
                                                            (canon %init_peer_id% $array-inline-96  #array-inline-96-0)
                                                           )
                                                          )
                                                          (par
                                                           (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-96-0])
                                                           (null)
                                                          )
                                                         )
                                                         (call %init_peer_id% ("json" "stringify") [#array-inline-96-0] ret-377)
                                                        )
                                                        (call %init_peer_id% (-spell_id-arg- "store_log") [ret-377] ret-378)
                                                       )
                                                      )
                                                      (seq
                                                       (ap :error: -if-error-)
                                                       (xor
                                                        (match :error:.$.error_code 10001
                                                         (null)
                                                        )
                                                        (fail -if-error-)
                                                       )
                                                      )
                                                     )
                                                    )
                                                   )
                                                   (ap ret-376.$.hex ret-376_flat)
                                                  )
                                                  (new -if-error-
                                                   (xor
                                                    (mismatch ret-376_flat []
                                                     (seq
                                                      (seq
                                                       (seq
                                                        (new %DealState_obj-3_map
                                                         (seq
                                                          (ap ("left_boundary" ret-376_flat.$.[0]) %DealState_obj-3_map)
                                                          (canon %init_peer_id% %DealState_obj-3_map  DealState_obj-3)
                                                         )
                                                        )
                                                        (call %init_peer_id% ("json" "stringify") [DealState_obj-3] ret-379)
                                                       )
                                                       (call %init_peer_id% ("op" "concat_strings") ["removed_state:" removed-0.$.deal_id] ret-380)
                                                      )
                                                      (call %init_peer_id% (-spell_id-arg- "set_string") [ret-380 ret-379] ret-381)
                                                     )
                                                    )
                                                    (seq
                                                     (ap :error: -if-error-)
                                                     (xor
                                                      (match :error:.$.error_code 10002
                                                       (null)
                                                      )
                                                      (fail -if-error-)
                                                     )
                                                    )
                                                   )
                                                  )
                                                 )
                                                )
                                                (seq
                                                 (ap :error: -if-error-)
                                                 (xor
                                                  (match :error:.$.error_code 10002
                                                   (null)
                                                  )
                                                  (fail -if-error-)
                                                 )
                                                )
                                               )
                                              )
                                             )
                                            )
                                            (seq
                                             (seq
                                              (ap :error: -else-error-)
                                              (xor
                                               (match :error:.$.error_code 10001
                                                (ap -if-error- -if-else-error-)
                                               )
                                               (ap -else-error- -if-else-error-)
                                              )
                                             )
                                             (fail -if-else-error-)
                                            )
                                           )
                                          )
                                         )
                                        )
                                       )
                                      )
                                     )
                                     (seq
                                      (seq
                                       (ap :error: -else-error-)
                                       (xor
                                        (match :error:.$.error_code 10001
                                         (ap -if-error- -if-else-error-)
                                        )
                                        (ap -else-error- -if-else-error-)
                                       )
                                      )
                                      (fail -if-else-error-)
                                     )
                                    )
                                   )
                                  )
                                 )
                                )
                               )
                              )
                              (next removed-0)
                             )
                             (null)
                            )
                           )
                           (seq
                            (seq
                             (ap :error: -else-error-)
                             (xor
                              (match :error:.$.error_code 10001
                               (ap -if-error- -if-else-error-)
                              )
                              (ap -else-error- -if-else-error-)
                             )
                            )
                            (fail -if-else-error-)
                           )
                          )
                         )
                        )
                       )
                      )
                     )
                    )
                   )
                   (seq
                    (ap :error: -if-error-)
                    (xor
                     (match :error:.$.error_code 10002
                      (null)
                     )
                     (fail -if-error-)
                    )
                   )
                  )
                 )
                )
               )
              )
              (seq
               (ap :error: -if-error-)
               (xor
                (match :error:.$.error_code 10002
                 (null)
                )
                (fail -if-error-)
               )
              )
             )
            )
           )
           (null)
          )
         )
         (xor
          (seq
           (seq
            (seq
             (new $known_txs
              (new $unknown_txs
               (seq
                (seq
                 (seq
                  (seq
                   (seq
                    (seq
                     (seq
                      (call %init_peer_id% (-spell_id-arg- "list_get_strings") ["worker_registration_txs"] ret-382)
                      (call %init_peer_id% (-spell_id-arg- "list_get_strings") ["worker_registration_txs_statuses"] ret-383)
                     )
                     (fold ret-383.$.value status_str-0
                      (seq
                       (xor
                        (seq
                         (seq
                          (call %init_peer_id% ("json" "parse") [status_str-0] ret-384)
                          (call %init_peer_id% ("json" "stringify") [ret-384.$.tx_info] ret-385)
                         )
                         (ap ret-385 $known_txs)
                        )
                        (seq
                         (seq
                          (seq
                           (new $array-inline-97
                            (seq
                             (seq
                              (seq
                               (seq
                                (ap "can't parse worker tx status:" $array-inline-97)
                                (ap status_str-0 $array-inline-97)
                               )
                               (ap "error:" $array-inline-97)
                              )
                              (ap :error: $array-inline-97)
                             )
                             (canon %init_peer_id% $array-inline-97  #array-inline-97-0)
                            )
                           )
                           (par
                            (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-97-0])
                            (null)
                           )
                          )
                          (call %init_peer_id% ("json" "stringify") [#array-inline-97-0] ret-386)
                         )
                         (call %init_peer_id% (-spell_id-arg- "store_log") [ret-386] ret-387)
                        )
                       )
                       (next status_str-0)
                      )
                      (null)
                     )
                    )
                    (canon %init_peer_id% $known_txs  #known_txs_canon)
                   )
                   (call %init_peer_id% ("array" "diff") [ret-382.$.value #known_txs_canon] ret-388)
                  )
                  (fold ret-388 tx-0
                   (seq
                    (xor
                     (seq
                      (call %init_peer_id% ("json" "parse") [tx-0] ret-389)
                      (ap ret-389 $unknown_txs)
                     )
                     (seq
                      (seq
                       (seq
                        (new $array-inline-98
                         (seq
                          (seq
                           (seq
                            (seq
                             (ap "can't parse worker tx info:" $array-inline-98)
                             (ap tx-0 $array-inline-98)
                            )
                            (ap "error:" $array-inline-98)
                           )
                           (ap :error: $array-inline-98)
                          )
                          (canon %init_peer_id% $array-inline-98  #array-inline-98-0)
                         )
                        )
                        (par
                         (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-98-0])
                         (null)
                        )
                       )
                       (call %init_peer_id% ("json" "stringify") [#array-inline-98-0] ret-390)
                      )
                      (call %init_peer_id% (-spell_id-arg- "store_log") [ret-390] ret-391)
                     )
                    )
                    (next tx-0)
                   )
                   (null)
                  )
                 )
                 (canon %init_peer_id% $unknown_txs  #-unknown_txs-fix-0)
                )
                (ap #-unknown_txs-fix-0 -unknown_txs-flat-0)
               )
              )
             )
             (ap -unknown_txs-flat-0 -unknown_txs-flat-0_to_functor)
            )
            (ap -unknown_txs-flat-0_to_functor.length -unknown_txs-flat-0_length)
           )
           (new -if-error-
            (xor
             (mismatch -unknown_txs-flat-0_length 0
              (seq
               (seq
                (seq
                 (seq
                  (seq
                   (seq
                    (new $array-inline-99
                     (seq
                      (seq
                       (seq
                        (seq
                         (seq
                          (ap -unknown_txs-flat-0 -unknown_txs-flat-0_to_functor-0)
                          (ap -unknown_txs-flat-0_to_functor-0.length -unknown_txs-flat-0_length-0)
                         )
                         (ap "tracking txs for" $array-inline-99)
                        )
                        (ap -unknown_txs-flat-0_length-0 $array-inline-99)
                       )
                       (ap "deals" $array-inline-99)
                      )
                      (canon %init_peer_id% $array-inline-99  #array-inline-99-0)
                     )
                    )
                    (par
                     (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-99-0])
                     (null)
                    )
                   )
                   (call %init_peer_id% ("json" "stringify") [#array-inline-99-0] ret-392)
                  )
                  (call %init_peer_id% (-spell_id-arg- "store_log") [ret-392] ret-393)
                 )
                 (call %init_peer_id% ("chain_connector" "get_tx_statuses") [-chain-arg-.$.api_endpoint -unknown_txs-flat-0] ret-394)
                )
                (xor
                 (match ret-394.$.success true
                  (ap false not-27)
                 )
                 (ap true not-27)
                )
               )
               (new -if-else-error-
                (new -else-error-
                 (new -if-error-
                  (xor
                   (match not-27 true
                    (seq
                     (seq
                      (seq
                       (new $array-inline-100
                        (seq
                         (seq
                          (ap "couldn't make batch jsonrpc request:" $array-inline-100)
                          (ap ret-394.$.error $array-inline-100)
                         )
                         (canon %init_peer_id% $array-inline-100  #array-inline-100-0)
                        )
                       )
                       (par
                        (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-100-0])
                        (null)
                       )
                      )
                      (call %init_peer_id% ("json" "stringify") [#array-inline-100-0] ret-395)
                     )
                     (call %init_peer_id% (-spell_id-arg- "store_log") [ret-395] ret-396)
                    )
                   )
                   (seq
                    (ap :error: -if-error-)
                    (xor
                     (match :error:.$.error_code 10001
                      (fold ret-394.$.results status-1
                       (seq
                        (seq
                         (xor
                          (match status-1.$.success true
                           (ap false not-28)
                          )
                          (ap true not-28)
                         )
                         (new -if-else-error-
                          (new -else-error-
                           (new -if-error-
                            (xor
                             (match not-28 true
                              (seq
                               (seq
                                (seq
                                 (seq
                                  (seq
                                   (new $array-inline-101
                                    (seq
                                     (seq
                                      (seq
                                       (seq
                                        (seq
                                         (ap "couldn't find tx status:" $array-inline-101)
                                         (ap "tx_hash:" $array-inline-101)
                                        )
                                        (ap status-1.$.tx.tx_hash $array-inline-101)
                                       )
                                       (ap "error:" $array-inline-101)
                                      )
                                      (ap status-1.$.error $array-inline-101)
                                     )
                                     (canon %init_peer_id% $array-inline-101  #array-inline-101-0)
                                    )
                                   )
                                   (call %init_peer_id% ("op" "concat_strings") ["decider deal_id=" status-1.$.tx.deal_id ": "] ret-397)
                                  )
                                  (par
                                   (call %init_peer_id% ("run-console" "print") [ret-397 #array-inline-101-0])
                                   (null)
                                  )
                                 )
                                 (call %init_peer_id% ("json" "stringify") [#array-inline-101-0] ret-398)
                                )
                                (call %init_peer_id% ("op" "concat_strings") [ret-397 ret-398] ret-399)
                               )
                               (call %init_peer_id% (-spell_id-arg- "store_log") [ret-399] ret-400)
                              )
                             )
                             (seq
                              (ap :error: -if-error-)
                              (xor
                               (match :error:.$.error_code 10001
                                (new -if-error-
                                 (xor
                                  (mismatch status-1.$.status "pending"
                                   (seq
                                    (seq
                                     (seq
                                      (seq
                                       (seq
                                        (new %WorkerTxStatus_obj_map
                                         (seq
                                          (seq
                                           (ap ("status" status-1.$.status) %WorkerTxStatus_obj_map)
                                           (ap ("tx_info" status-1.$.tx) %WorkerTxStatus_obj_map)
                                          )
                                          (canon %init_peer_id% %WorkerTxStatus_obj_map  WorkerTxStatus_obj)
                                         )
                                        )
                                        (call %init_peer_id% ("json" "stringify") [WorkerTxStatus_obj] ret-401)
                                       )
                                       (call %init_peer_id% (-spell_id-arg- "list_push_string") ["worker_registration_txs_statuses" ret-401] ret-402)
                                      )
                                      (xor
                                       (match ret-402.$.success true
                                        (ap false not-29)
                                       )
                                       (ap true not-29)
                                      )
                                     )
                                     (new -if-error-
                                      (xor
                                       (match not-29 true
                                        (seq
                                         (seq
                                          (seq
                                           (new $array-inline-102
                                            (seq
                                             (seq
                                              (seq
                                               (seq
                                                (ap "can't store value in list by key" $array-inline-102)
                                                (ap "worker_registration_txs_statuses" $array-inline-102)
                                               )
                                               (ap "error:" $array-inline-102)
                                              )
                                              (ap ret-402.$.error $array-inline-102)
                                             )
                                             (canon %init_peer_id% $array-inline-102  #array-inline-102-0)
                                            )
                                           )
                                           (par
                                            (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-102-0])
                                            (null)
                                           )
                                          )
                                          (call %init_peer_id% ("json" "stringify") [#array-inline-102-0] ret-403)
                                         )
                                         (call %init_peer_id% (-spell_id-arg- "store_log") [ret-403] ret-404)
                                        )
                                       )
                                       (seq
                                        (ap :error: -if-error-)
                                        (xor
                                         (match :error:.$.error_code 10001
                                          (null)
                                         )
                                         (fail -if-error-)
                                        )
                                       )
                                      )
                                     )
                                    )
                                    (new -if-error-
                                     (xor
                                      (match status-1.$.status "failed"
                                       (seq
                                        (seq
                                         (seq
                                          (seq
                                           (seq
                                            (seq
                                             (seq
                                              (seq
                                               (seq
                                                (seq
                                                 (seq
                                                  (seq
                                                   (seq
                                                    (new $option-inline
                                                     (seq
                                                      (xor
                                                       (ap status-1.$.tx.tx_hash $option-inline)
                                                       (null)
                                                      )
                                                      (canon %init_peer_id% $option-inline  #option-inline-0)
                                                     )
                                                    )
                                                    (new %TxFailed_obj-0_map
                                                     (seq
                                                      (seq
                                                       (ap ("block_number" status-1.$.block_number) %TxFailed_obj-0_map)
                                                       (ap ("tx_hash" #option-inline-0) %TxFailed_obj-0_map)
                                                      )
                                                      (canon %init_peer_id% %TxFailed_obj-0_map  TxFailed_obj-0)
                                                     )
                                                    )
                                                   )
                                                   (new %FailedDealError_obj-1_map
                                                    (seq
                                                     (seq
                                                      (ap ("content" TxFailed_obj-0) %FailedDealError_obj-1_map)
                                                      (ap ("type" "TxFailed") %FailedDealError_obj-1_map)
                                                     )
                                                     (canon %init_peer_id% %FailedDealError_obj-1_map  FailedDealError_obj-1)
                                                    )
                                                   )
                                                  )
                                                  (new %FailedDeal_obj-1_map
                                                   (seq
                                                    (seq
                                                     (seq
                                                      (ap ("deal_id" status-1.$.tx.deal_id) %FailedDeal_obj-1_map)
                                                      (ap ("message" "transaction failed") %FailedDeal_obj-1_map)
                                                     )
                                                     (ap ("payload" FailedDealError_obj-1) %FailedDeal_obj-1_map)
                                                    )
                                                    (canon %init_peer_id% %FailedDeal_obj-1_map  FailedDeal_obj-1)
                                                   )
                                                  )
                                                 )
                                                 (call %init_peer_id% ("json" "stringify") [FailedDeal_obj-1] ret-405)
                                                )
                                                (call %init_peer_id% (-spell_id-arg- "list_push_string") ["failed_deals" ret-405] ret-406)
                                               )
                                               (xor
                                                (match ret-406.$.success true
                                                 (ap false not-30)
                                                )
                                                (ap true not-30)
                                               )
                                              )
                                              (new -if-error-
                                               (xor
                                                (match not-30 true
                                                 (seq
                                                  (seq
                                                   (seq
                                                    (new $array-inline-103
                                                     (seq
                                                      (seq
                                                       (seq
                                                        (seq
                                                         (ap "can't store value in list by key" $array-inline-103)
                                                         (ap "failed_deals" $array-inline-103)
                                                        )
                                                        (ap "error:" $array-inline-103)
                                                       )
                                                       (ap ret-406.$.error $array-inline-103)
                                                      )
                                                      (canon %init_peer_id% $array-inline-103  #array-inline-103-0)
                                                     )
                                                    )
                                                    (par
                                                     (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-103-0])
                                                     (null)
                                                    )
                                                   )
                                                   (call %init_peer_id% ("json" "stringify") [#array-inline-103-0] ret-407)
                                                  )
                                                  (call %init_peer_id% (-spell_id-arg- "store_log") [ret-407] ret-408)
                                                 )
                                                )
                                                (seq
                                                 (ap :error: -if-error-)
                                                 (xor
                                                  (match :error:.$.error_code 10001
                                                   (null)
                                                  )
                                                  (fail -if-error-)
                                                 )
                                                )
                                               )
                                              )
                                             )
                                             (new $array-inline-104
                                              (seq
                                               (seq
                                                (seq
                                                 (seq
                                                  (ap "transaction failed, tx_hash:" $array-inline-104)
                                                  (ap status-1.$.tx.tx_hash $array-inline-104)
                                                 )
                                                 (ap "block_number:" $array-inline-104)
                                                )
                                                (ap status-1.$.block_number $array-inline-104)
                                               )
                                               (canon %init_peer_id% $array-inline-104  #array-inline-104-0)
                                              )
                                             )
                                            )
                                            (call %init_peer_id% ("op" "concat_strings") ["decider deal_id=" status-1.$.tx.deal_id ": "] ret-409)
                                           )
                                           (par
                                            (call %init_peer_id% ("run-console" "print") [ret-409 #array-inline-104-0])
                                            (null)
                                           )
                                          )
                                          (call %init_peer_id% ("json" "stringify") [#array-inline-104-0] ret-410)
                                         )
                                         (call %init_peer_id% ("op" "concat_strings") [ret-409 ret-410] ret-411)
                                        )
                                        (call %init_peer_id% (-spell_id-arg- "store_log") [ret-411] ret-412)
                                       )
                                      )
                                      (seq
                                       (ap :error: -if-error-)
                                       (xor
                                        (match :error:.$.error_code 10001
                                         (null)
                                        )
                                        (fail -if-error-)
                                       )
                                      )
                                     )
                                    )
                                   )
                                  )
                                  (seq
                                   (ap :error: -if-error-)
                                   (xor
                                    (match :error:.$.error_code 10002
                                     (null)
                                    )
                                    (fail -if-error-)
                                   )
                                  )
                                 )
                                )
                               )
                               (seq
                                (seq
                                 (ap :error: -else-error-)
                                 (xor
                                  (match :error:.$.error_code 10001
                                   (ap -if-error- -if-else-error-)
                                  )
                                  (ap -else-error- -if-else-error-)
                                 )
                                )
                                (fail -if-else-error-)
                               )
                              )
                             )
                            )
                           )
                          )
                         )
                        )
                        (next status-1)
                       )
                       (null)
                      )
                     )
                     (seq
                      (seq
                       (ap :error: -else-error-)
                       (xor
                        (match :error:.$.error_code 10001
                         (ap -if-error- -if-else-error-)
                        )
                        (ap -else-error- -if-else-error-)
                       )
                      )
                      (fail -if-else-error-)
                     )
                    )
                   )
                  )
                 )
                )
               )
              )
             )
             (seq
              (ap :error: -if-error-)
              (xor
               (match :error:.$.error_code 10002
                (null)
               )
               (fail -if-error-)
              )
             )
            )
           )
          )
          (null)
         )
        )
        (xor
         (seq
          (new $result-16
           (seq
            (seq
             (seq
              (seq
               (call %init_peer_id% (-spell_id-arg- "get_string") ["last_seen_block"] ret-413)
               (xor
                (match ret-413.$.success true
                 (ap false not-31)
                )
                (ap true not-31)
               )
              )
              (new -if-else-error-
               (new -else-error-
                (new -if-error-
                 (xor
                  (match not-31 true
                   (seq
                    (seq
                     (seq
                      (new $array-inline-105
                       (seq
                        (seq
                         (seq
                          (ap "get_string failed" $array-inline-105)
                          (ap "last_seen_block" $array-inline-105)
                         )
                         (ap ret-413.$.error $array-inline-105)
                        )
                        (canon %init_peer_id% $array-inline-105  #array-inline-105-0)
                       )
                      )
                      (par
                       (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-105-0])
                       (null)
                      )
                     )
                     (call %init_peer_id% ("json" "stringify") [#array-inline-105-0] ret-414)
                    )
                    (call %init_peer_id% (-spell_id-arg- "store_log") [ret-414] ret-415)
                   )
                  )
                  (seq
                   (ap :error: -if-error-)
                   (xor
                    (match :error:.$.error_code 10001
                     (seq
                      (xor
                       (match ret-413.$.absent true
                        (ap false not-32)
                       )
                       (ap true not-32)
                      )
                      (new -if-error-
                       (xor
                        (match not-32 true
                         (ap ret-413.$.value $result-16)
                        )
                        (seq
                         (ap :error: -if-error-)
                         (xor
                          (match :error:.$.error_code 10001
                           (null)
                          )
                          (fail -if-error-)
                         )
                        )
                       )
                      )
                     )
                    )
                    (seq
                     (seq
                      (ap :error: -else-error-)
                      (xor
                       (match :error:.$.error_code 10001
                        (ap -if-error- -if-else-error-)
                       )
                       (ap -else-error- -if-else-error-)
                      )
                     )
                     (fail -if-else-error-)
                    )
                   )
                  )
                 )
                )
               )
              )
             )
             (canon %init_peer_id% $result-16  #-result-fix-0-4)
            )
            (ap #-result-fix-0-4 -result-flat-0-4)
           )
          )
          (new -if-error-
           (xor
            (mismatch -result-flat-0-4 []
             (seq
              (seq
               (seq
                (seq
                 (seq
                  (seq
                   (call %init_peer_id% ("chain_connector" "hex_diff") [-result-flat-0-4.$.[0] -latest-flat-0.$.[0]] ret-416)
                   (new $result-17
                    (seq
                     (seq
                      (call %init_peer_id% (-spell_id-arg- "get_u32") ["counter"] ret-417)
                      (new -if-else-error-
                       (new -else-error-
                        (new -if-error-
                         (xor
                          (match ret-417.$.success true
                           (ap ret-417.$.value $result-17)
                          )
                          (seq
                           (ap :error: -if-error-)
                           (xor
                            (match :error:.$.error_code 10001
                             (ap 0 $result-17)
                            )
                            (seq
                             (seq
                              (ap :error: -else-error-)
                              (xor
                               (match :error:.$.error_code 10001
                                (ap -if-error- -if-else-error-)
                               )
                               (ap -else-error- -if-else-error-)
                              )
                             )
                             (fail -if-else-error-)
                            )
                           )
                          )
                         )
                        )
                       )
                      )
                     )
                     (new $result-17_test
                      (seq
                       (seq
                        (fold $result-17 result-17_fold_var
                         (seq
                          (seq
                           (ap result-17_fold_var $result-17_test)
                           (canon %init_peer_id% $result-17_test  #result-17_iter_canon)
                          )
                          (xor
                           (match #result-17_iter_canon.length 1
                            (null)
                           )
                           (next result-17_fold_var)
                          )
                         )
                         (never)
                        )
                        (canon %init_peer_id% $result-17_test  #result-17_result_canon)
                       )
                       (ap #result-17_result_canon result-17_gate)
                      )
                     )
                    )
                   )
                  )
                  (new %SyncInfo_obj_map
                   (seq
                    (seq
                     (ap ("blocks_diff" ret-416) %SyncInfo_obj_map)
                     (ap ("run_updated" result-17_gate.$.[0]) %SyncInfo_obj_map)
                    )
                    (canon %init_peer_id% %SyncInfo_obj_map  SyncInfo_obj)
                   )
                  )
                 )
                 (call %init_peer_id% ("json" "stringify") [SyncInfo_obj] ret-418)
                )
                (call %init_peer_id% (-spell_id-arg- "set_string") ["sync_info" ret-418] ret-419)
               )
               (xor
                (match ret-419.$.success true
                 (ap false not-33)
                )
                (ap true not-33)
               )
              )
              (new -if-error-
               (xor
                (match not-33 true
                 (seq
                  (seq
                   (seq
                    (new $array-inline-106
                     (seq
                      (seq
                       (seq
                        (seq
                         (ap "can't updated sync state" $array-inline-106)
                         (ap SyncInfo_obj $array-inline-106)
                        )
                        (ap "error" $array-inline-106)
                       )
                       (ap ret-419.$.error $array-inline-106)
                      )
                      (canon %init_peer_id% $array-inline-106  #array-inline-106-0)
                     )
                    )
                    (par
                     (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-106-0])
                     (null)
                    )
                   )
                   (call %init_peer_id% ("json" "stringify") [#array-inline-106-0] ret-420)
                  )
                  (call %init_peer_id% (-spell_id-arg- "store_log") [ret-420] ret-421)
                 )
                )
                (seq
                 (ap :error: -if-error-)
                 (xor
                  (match :error:.$.error_code 10001
                   (null)
                  )
                  (fail -if-error-)
                 )
                )
               )
              )
             )
            )
            (seq
             (ap :error: -if-error-)
             (xor
              (match :error:.$.error_code 10002
               (null)
              )
              (fail -if-error-)
             )
            )
           )
          )
         )
         (null)
        )
       )
       (xor
        (seq
         (seq
          (seq
           (seq
            (seq
             (call %init_peer_id% (-spell_id-arg- "get_mailbox") [] ret-422)
             (new $array-inline-107
              (seq
               (seq
                (ap "mailbox" $array-inline-107)
                (ap ret-422 $array-inline-107)
               )
               (canon %init_peer_id% $array-inline-107  #array-inline-107-0)
              )
             )
            )
            (par
             (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-107-0])
             (null)
            )
           )
           (call %init_peer_id% ("json" "stringify") [#array-inline-107-0] ret-423)
          )
          (call %init_peer_id% (-spell_id-arg- "store_log") [ret-423] ret-424)
         )
         (new -if-error-
          (xor
           (match ret-422.$.success true
            (fold ret-422.$.messages msg-233-0
             (seq
              (xor
               (seq
                (seq
                 (seq
                  (seq
                   (seq
                    (seq
                     (seq
                      (seq
                       (seq
                        (seq
                         (seq
                          (call %init_peer_id% ("json" "parse") [msg-233-0.$.message] ret-425)
                          (new -if-error-
                           (xor
                            (mismatch ret-425.$.remove []
                             (seq
                              (seq
                               (seq
                                (seq
                                 (ap ret-425.$.remove ret-425_flat)
                                 (new $array-inline-108
                                  (seq
                                   (seq
                                    (seq
                                     (ap "called remove worker via mailbox" $array-inline-108)
                                     (ap ret-425_flat.$.[0].host_id $array-inline-108)
                                    )
                                    (ap ret-425_flat.$.[0].worker_id $array-inline-108)
                                   )
                                   (canon %init_peer_id% $array-inline-108  #array-inline-108-0)
                                  )
                                 )
                                )
                                (par
                                 (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-108-0])
                                 (null)
                                )
                               )
                               (call %init_peer_id% ("json" "stringify") [#array-inline-108-0] ret-426)
                              )
                              (call %init_peer_id% ("decider" "store_log") [ret-426] ret-427)
                             )
                            )
                            (seq
                             (ap :error: -if-error-)
                             (xor
                              (match :error:.$.error_code 10002
                               (null)
                              )
                              (fail -if-error-)
                             )
                            )
                           )
                          )
                         )
                         (par
                          (call %init_peer_id% ("run-console" "print") ["decider" "will pop"])
                          (null)
                         )
                        )
                        (call %init_peer_id% ("json" "stringify") ["will pop"] ret-428)
                       )
                       (call %init_peer_id% (-spell_id-arg- "store_log") [ret-428] ret-429)
                      )
                      (call %init_peer_id% (-spell_id-arg- "pop_mailbox") [] ret-430)
                     )
                     (ap ret-430.$.message ret-430_flat)
                    )
                    (ap ret-430_flat ret-430_flat_to_functor)
                   )
                   (ap ret-430_flat_to_functor.length ret-430_flat_length)
                  )
                  (xor
                   (match ret-430_flat_length 0
                    (ap true eq)
                   )
                   (ap false eq)
                  )
                 )
                 (xor
                  (match eq true
                   (ap eq or)
                  )
                  (seq
                   (xor
                    (mismatch ret-430.$.message.[0].message msg-233-0.$.message
                     (ap true neq)
                    )
                    (ap false neq)
                   )
                   (ap neq or)
                  )
                 )
                )
                (new -if-error-
                 (xor
                  (match or true
                   (seq
                    (seq
                     (seq
                      (new $array-inline-109
                       (seq
                        (seq
                         (seq
                          (seq
                           (ap "broken invariant, expected" $array-inline-109)
                           (ap msg-233-0 $array-inline-109)
                          )
                          (ap "popped" $array-inline-109)
                         )
                         (ap ret-430 $array-inline-109)
                        )
                        (canon %init_peer_id% $array-inline-109  #array-inline-109-0)
                       )
                      )
                      (par
                       (call %init_peer_id% ("run-console" "print") ["decider" #array-inline-109-0])
                       (null)
                      )
                     )
                     (call %init_peer_id% ("json" "stringify") [#array-inline-109-0] ret-431)
                    )
                    (call %init_peer_id% (-spell_id-arg- "store_log") [ret-431] ret-432)
                   )
                  )
                  (seq
                   (ap :error: -if-error-)
                   (xor
                    (match :error:.$.error_code 10001
                     (null)
                    )
                    (fail -if-error-)
                   )
                  )
                 )
                )
               )
               (null)
              )
              (next msg-233-0)
             )
             (null)
            )
           )
           (seq
            (ap :error: -if-error-)
            (xor
             (match :error:.$.error_code 10001
              (null)
             )
             (fail -if-error-)
            )
           )
          )
         )
        )
        (null)
       )
      )
     )
     (seq
      (ap :error: -if-error-)
      (xor
       (match :error:.$.error_code 10002
        (null)
       )
       (fail -if-error-)
      )
     )
    )
   )
  )
  (call %init_peer_id% ("callbackSrv" "response") [])
 )
 (call %init_peer_id% ("errorHandlingSrv" "error") [:error: 0])
)
`;

export type MainArgChain = { network_id: number; wallet_key: string; workers_gas: number; api_endpoint: string; matcher: string; }
export type MainArgWorker_settings = { config: { blockchain: { end_block: number; start_block: number; }; clock: { end_sec: number; period_sec: number; start_sec: number; }; connections: { connect: boolean; disconnect: boolean; }; }; ipfs: string; script: string; }

export type MainParams = [spell_id: string, chain: MainArgChain, worker_settings: MainArgWorker_settings, config?: {ttl?: number}] | [peer: IFluenceClient$$, spell_id: string, chain: MainArgChain, worker_settings: MainArgWorker_settings, config?: {ttl?: number}];

export type MainResult = Promise<void>;

export function main(...args: MainParams): MainResult {
    return callFunction$$(
        args,
        {
    "functionName": "main",
    "arrow": {
        "domain": {
            "fields": {
                "spell_id": {
                    "name": "string",
                    "tag": "scalar"
                },
                "chain": {
                    "name": "ChainInfo",
                    "fields": {
                        "network_id": {
                            "name": "u64",
                            "tag": "scalar"
                        },
                        "wallet_key": {
                            "name": "string",
                            "tag": "scalar"
                        },
                        "workers_gas": {
                            "name": "u64",
                            "tag": "scalar"
                        },
                        "api_endpoint": {
                            "name": "string",
                            "tag": "scalar"
                        },
                        "matcher": {
                            "name": "string",
                            "tag": "scalar"
                        }
                    },
                    "tag": "struct"
                },
                "worker_settings": {
                    "name": "WorkerSettings",
                    "fields": {
                        "config": {
                            "name": "TriggerConfig",
                            "fields": {
                                "blockchain": {
                                    "name": "BlockChainConfig",
                                    "fields": {
                                        "end_block": {
                                            "name": "u32",
                                            "tag": "scalar"
                                        },
                                        "start_block": {
                                            "name": "u32",
                                            "tag": "scalar"
                                        }
                                    },
                                    "tag": "struct"
                                },
                                "clock": {
                                    "name": "ClockConfig",
                                    "fields": {
                                        "end_sec": {
                                            "name": "u32",
                                            "tag": "scalar"
                                        },
                                        "period_sec": {
                                            "name": "u32",
                                            "tag": "scalar"
                                        },
                                        "start_sec": {
                                            "name": "u32",
                                            "tag": "scalar"
                                        }
                                    },
                                    "tag": "struct"
                                },
                                "connections": {
                                    "name": "ConnectionPoolConfig",
                                    "fields": {
                                        "connect": {
                                            "name": "bool",
                                            "tag": "scalar"
                                        },
                                        "disconnect": {
                                            "name": "bool",
                                            "tag": "scalar"
                                        }
                                    },
                                    "tag": "struct"
                                }
                            },
                            "tag": "struct"
                        },
                        "ipfs": {
                            "name": "string",
                            "tag": "scalar"
                        },
                        "script": {
                            "name": "string",
                            "tag": "scalar"
                        }
                    },
                    "tag": "struct"
                }
            },
            "tag": "labeledProduct"
        },
        "codomain": {
            "tag": "nil"
        },
        "tag": "arrow"
    },
    "names": {
        "relay": "-relay-",
        "getDataSrv": "getDataSrv",
        "callbackSrv": "callbackSrv",
        "responseSrv": "callbackSrv",
        "responseFnName": "response",
        "errorHandlingSrv": "errorHandlingSrv",
        "errorFnName": "error"
    }
},
        main_script
    );
}
