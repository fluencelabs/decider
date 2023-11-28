try:
    -relay- <- call %init_peer_id% ("getDataSrv", "-relay-") []
    spell_id <- call %init_peer_id% ("getDataSrv", "spell_id") []
    info <- call %init_peer_id% ("getDataSrv", "info") []
    from_block <- call %init_peer_id% ("getDataSrv", "from_block") []
    try:
        new $from_block_init:
            new $result-0:
                counter <- call %init_peer_id% (spell_id, "get_u32") ["counter"]
                try:
                    match counter.$.success true:
                        ap counter.$.num $result-0
                catch:
                    ap 0 $result-0
                new $result-0_test:
                    result-0_incr <- call %init_peer_id% ("math", "add") [0, 1]
                    fold $result-0 result-0_fold_var:
                        ap result-0_fold_var $result-0_test
                        canon %init_peer_id% $result-0_test #result-0_iter_canon
                        try:
                            match #result-0_iter_canon.length result-0_incr:
                                null
                        catch:
                            next result-0_fold_var
                    last:
                        never
                    canon %init_peer_id% $result-0_test #result-0_result_canon
                    ap #result-0_result_canon result-0_gate
            is_first_iteration <- call %init_peer_id% ("cmp", "lte") [result-0_gate.$.[0], 1]
            new $result-1:
                try:
                    match from_block "latest":
                        ap true $result-1
                catch:
                    ap false $result-1
                new $result-1_test:
                    result-1_incr <- call %init_peer_id% ("math", "add") [0, 1]
                    fold $result-1 result-1_fold_var:
                        ap result-1_fold_var $result-1_test
                        canon %init_peer_id% $result-1_test #result-1_iter_canon
                        try:
                            match #result-1_iter_canon.length result-1_incr:
                                null
                        catch:
                            next result-1_fold_var
                    last:
                        never
                    canon %init_peer_id% $result-1_test #result-1_result_canon
                    ap #result-1_result_canon result-1_gate
            try:
                new $res:
                    try:
                        match is_first_iteration true:
                            ap true $res
                    catch:
                        ap result-1_gate.$.[0] $res
                    new $res_test:
                        res_incr <- call %init_peer_id% ("math", "add") [0, 1]
                        fold $res res_fold_var:
                            ap res_fold_var $res_test
                            canon %init_peer_id% $res_test #res_iter_canon
                            try:
                                match #res_iter_canon.length res_incr:
                                    null
                            catch:
                                next res_fold_var
                        last:
                            never
                        canon %init_peer_id% $res_test #res_result_canon
                        ap #res_result_canon res_gate
                match res_gate.$.[0] true:
                    try:
                        bnumber <- call %init_peer_id% ("fluence_aurora_connector", "latest_block_number") [info.$.api_endpoint]
                        try:
                            match bnumber.$.success true:
                                try:
                                    new $array-inline:
                                        ap "update from_block to the latest block: [init, new]" $array-inline
                                        ap from_block $array-inline
                                        ap bnumber.$.result $array-inline
                                        canon %init_peer_id% $array-inline #array-inline-0
                                    title <- call %init_peer_id% ("op", "concat_strings") ["decider <", spell_id, ">"]
                                    call %init_peer_id% ("run-console", "print") [title, #array-inline-0]
                                    msg_str <- call %init_peer_id% ("json", "stringify") [#array-inline-0]
                                    call %init_peer_id% (spell_id, "list_push_string") ["logs", msg_str]
                                    bnumber_str <- call %init_peer_id% ("json", "stringify") [bnumber.$.result]
                                    call %init_peer_id% (spell_id, "set_string") ["from_block", bnumber_str]
                                    ap bnumber.$.result $from_block_init
                                catch:
                                    call %init_peer_id% ("errorHandlingSrv", "error") [%last_error%, 1]
                        catch:
                            call %init_peer_id% ("op", "noop") []
                    catch:
                        call %init_peer_id% ("errorHandlingSrv", "error") [%last_error%, 2]
            catch:
                call %init_peer_id% ("op", "noop") []
            ap from_block $from_block_init
            new $from_block_init_test:
                from_block_init_incr <- call %init_peer_id% ("math", "add") [0, 1]
                fold $from_block_init from_block_init_fold_var:
                    ap from_block_init_fold_var $from_block_init_test
                    canon %init_peer_id% $from_block_init_test #from_block_init_iter_canon
                    try:
                        match #from_block_init_iter_canon.length from_block_init_incr:
                            null
                    catch:
                        next from_block_init_fold_var
                last:
                    never
                canon %init_peer_id% $from_block_init_test #from_block_init_result_canon
                ap #from_block_init_result_canon from_block_init_gate
            null
            result <- call %init_peer_id% ("fluence_aurora_connector", "poll_deals") [info.$.api_endpoint, info.$.address, from_block_init_gate.$.[0]]
            try:
                match result.$.success false:
                    try:
                        new $array-inline-1:
                            ap "can't receive info about new deals" $array-inline-1
                            ap result.$.error.[0] $array-inline-1
                            canon %init_peer_id% $array-inline-1 #array-inline-1-0
                        title-0 <- call %init_peer_id% ("op", "concat_strings") ["decider <", spell_id, ">"]
                        call %init_peer_id% ("run-console", "print") [title-0, #array-inline-1-0]
                        msg_str-0 <- call %init_peer_id% ("json", "stringify") [#array-inline-1-0]
                        call %init_peer_id% (spell_id, "list_push_string") ["logs", msg_str-0]
                    catch:
                        call %init_peer_id% ("errorHandlingSrv", "error") [%last_error%, 3]
            catch:
                fold result.$.result deal-0:
                    new $array-inline-2:
                        ap "found deal" $array-inline-2
                        ap deal-0.$.info.deal_id $array-inline-2
                        ap "from block" $array-inline-2
                        ap deal-0.$.block_number $array-inline-2
                        canon %init_peer_id% $array-inline-2 #array-inline-2-0
                    title-1 <- call %init_peer_id% ("op", "concat_strings") ["decider <", spell_id, ">"]
                    call %init_peer_id% ("run-console", "print") [title-1, #array-inline-2-0]
                    msg_str-1 <- call %init_peer_id% ("json", "stringify") [#array-inline-2-0]
                    call %init_peer_id% (spell_id, "list_push_string") ["logs", msg_str-1]
                    try:
                        new $result-2:
                            try:
                                worker_id <- call %init_peer_id% ("worker", "get_peer_id") [deal-0.$.info.deal_id]
                                ap true $result-2
                            catch:
                                ap false $result-2
                            new $result-2_test:
                                result-2_incr <- call %init_peer_id% ("math", "add") [0, 1]
                                fold $result-2 result-2_fold_var:
                                    ap result-2_fold_var $result-2_test
                                    canon %init_peer_id% $result-2_test #result-2_iter_canon
                                    try:
                                        match #result-2_iter_canon.length result-2_incr:
                                            null
                                    catch:
                                        next result-2_fold_var
                                last:
                                    never
                                canon %init_peer_id% $result-2_test #result-2_result_canon
                                ap #result-2_result_canon result-2_gate
                        match result-2_gate.$.[0] true:
                            try:
                                new $array-inline-3:
                                    ap "worker for deal" $array-inline-3
                                    ap deal-0.$.info.deal_id $array-inline-3
                                    ap "already created" $array-inline-3
                                    canon %init_peer_id% $array-inline-3 #array-inline-3-0
                                title-2 <- call %init_peer_id% ("op", "concat_strings") ["decider <", spell_id, ">"]
                                call %init_peer_id% ("run-console", "print") [title-2, #array-inline-3-0]
                                msg_str-2 <- call %init_peer_id% ("json", "stringify") [#array-inline-3-0]
                                call %init_peer_id% (spell_id, "list_push_string") ["logs", msg_str-2]
                            catch:
                                call %init_peer_id% ("errorHandlingSrv", "error") [%last_error%, 4]
                    catch:
                        try:
                            match true false:
                                try:
                                    new $array-inline-4:
                                        ap "skipping deal for deal id" $array-inline-4
                                        ap deal-0.$.info.deal_id $array-inline-4
                                        ap "from_block" $array-inline-4
                                        ap deal-0.$.block_number $array-inline-4
                                        canon %init_peer_id% $array-inline-4 #array-inline-4-0
                                    title-3 <- call %init_peer_id% ("op", "concat_strings") ["decider <", spell_id, ">"]
                                    call %init_peer_id% ("run-console", "print") [title-3, #array-inline-4-0]
                                    msg_str-3 <- call %init_peer_id% ("json", "stringify") [#array-inline-4-0]
                                    call %init_peer_id% (spell_id, "list_push_string") ["logs", msg_str-3]
                                catch:
                                    call %init_peer_id% ("errorHandlingSrv", "error") [%last_error%, 5]
                        catch:
                            new $array-inline-5:
                                ap "joining the deal" $array-inline-5
                                ap deal-0.$.info.deal_id $array-inline-5
                                ap "from_block" $array-inline-5
                                ap deal-0.$.block_number $array-inline-5
                                canon %init_peer_id% $array-inline-5 #array-inline-5-0
                            title-4 <- call %init_peer_id% ("op", "concat_strings") ["decider <", spell_id, ">"]
                            call %init_peer_id% ("run-console", "print") [title-4, #array-inline-5-0]
                            msg_str-4 <- call %init_peer_id% ("json", "stringify") [#array-inline-5-0]
                            call %init_peer_id% (spell_id, "list_push_string") ["logs", msg_str-4]
                            null
                            new $status:
                                new $settings:
                                    new $result-3:
                                        new $status-0:
                                            new $result-4:
                                                value <- call %init_peer_id% (spell_id, "get_string") ["worker_script"]
                                                try:
                                                    match value.$.success false:
                                                        ap false $status-0
                                                catch:
                                                    try:
                                                        match value.$.absent true:
                                                            ap false $status-0
                                                    catch:
                                                        ap value.$.str $result-4
                                                        ap true $status-0
                                                canon %init_peer_id% $result-4 #-result-fix-0
                                                ap #-result-fix-0 -result-flat-0
                                                new $status-0_test:
                                                    status-0_incr <- call %init_peer_id% ("math", "add") [0, 1]
                                                    fold $status-0 status-0_fold_var:
                                                        ap status-0_fold_var $status-0_test
                                                        canon %init_peer_id% $status-0_test #status-0_iter_canon
                                                        try:
                                                            match #status-0_iter_canon.length status-0_incr:
                                                                null
                                                        catch:
                                                            next status-0_fold_var
                                                    last:
                                                        never
                                                    canon %init_peer_id% $status-0_test #status-0_result_canon
                                                    ap #status-0_result_canon status-0_gate
                                        new $status-1:
                                            new $result-5:
                                                value-0 <- call %init_peer_id% (spell_id, "get_string") ["worker_config"]
                                                try:
                                                    match value-0.$.success false:
                                                        ap false $status-1
                                                catch:
                                                    try:
                                                        match value-0.$.absent true:
                                                            ap false $status-1
                                                    catch:
                                                        ap value-0.$.str $result-5
                                                        ap true $status-1
                                                canon %init_peer_id% $result-5 #-result-fix-0-0
                                                ap #-result-fix-0-0 -result-flat-0-0
                                                new $status-1_test:
                                                    status-1_incr <- call %init_peer_id% ("math", "add") [0, 1]
                                                    fold $status-1 status-1_fold_var:
                                                        ap status-1_fold_var $status-1_test
                                                        canon %init_peer_id% $status-1_test #status-1_iter_canon
                                                        try:
                                                            match #status-1_iter_canon.length status-1_incr:
                                                                null
                                                        catch:
                                                            next status-1_fold_var
                                                    last:
                                                        never
                                                    canon %init_peer_id% $status-1_test #status-1_result_canon
                                                    ap #status-1_result_canon status-1_gate
                                        new $status-2:
                                            new $result-6:
                                                value-1 <- call %init_peer_id% (spell_id, "get_string") ["worker_ipfs"]
                                                try:
                                                    match value-1.$.success false:
                                                        ap false $status-2
                                                catch:
                                                    try:
                                                        match value-1.$.absent true:
                                                            ap false $status-2
                                                    catch:
                                                        ap value-1.$.str $result-6
                                                        ap true $status-2
                                                canon %init_peer_id% $result-6 #-result-fix-0-1
                                                ap #-result-fix-0-1 -result-flat-0-1
                                                new $status-2_test:
                                                    status-2_incr <- call %init_peer_id% ("math", "add") [0, 1]
                                                    fold $status-2 status-2_fold_var:
                                                        ap status-2_fold_var $status-2_test
                                                        canon %init_peer_id% $status-2_test #status-2_iter_canon
                                                        try:
                                                            match #status-2_iter_canon.length status-2_incr:
                                                                null
                                                        catch:
                                                            next status-2_fold_var
                                                    last:
                                                        never
                                                    canon %init_peer_id% $status-2_test #status-2_result_canon
                                                    ap #status-2_result_canon status-2_gate
                                        try:
                                            match status-0_gate.$.[0] false:
                                                try:
                                                    new $array-inline-6:
                                                        ap "worker_script doesn't set" $array-inline-6
                                                        canon %init_peer_id% $array-inline-6 #array-inline-6-0
                                                    title-5 <- call %init_peer_id% ("op", "concat_strings") ["decider <", spell_id, ">"]
                                                    call %init_peer_id% ("run-console", "print") [title-5, #array-inline-6-0]
                                                    msg_str-5 <- call %init_peer_id% ("json", "stringify") [#array-inline-6-0]
                                                    call %init_peer_id% (spell_id, "list_push_string") ["logs", msg_str-5]
                                                    ap false $result-3
                                                catch:
                                                    call %init_peer_id% ("errorHandlingSrv", "error") [%last_error%, 6]
                                        catch:
                                            try:
                                                match status-1_gate.$.[0] false:
                                                    try:
                                                        new $array-inline-7:
                                                            ap "worker_config doesn't set" $array-inline-7
                                                            canon %init_peer_id% $array-inline-7 #array-inline-7-0
                                                        title-6 <- call %init_peer_id% ("op", "concat_strings") ["decider <", spell_id, ">"]
                                                        call %init_peer_id% ("run-console", "print") [title-6, #array-inline-7-0]
                                                        msg_str-6 <- call %init_peer_id% ("json", "stringify") [#array-inline-7-0]
                                                        call %init_peer_id% (spell_id, "list_push_string") ["logs", msg_str-6]
                                                        ap false $result-3
                                                    catch:
                                                        call %init_peer_id% ("errorHandlingSrv", "error") [%last_error%, 7]
                                            catch:
                                                try:
                                                    match status-2_gate.$.[0] false:
                                                        try:
                                                            new $array-inline-8:
                                                                ap "worker_ipfs doesn't set" $array-inline-8
                                                                canon %init_peer_id% $array-inline-8 #array-inline-8-0
                                                            title-7 <- call %init_peer_id% ("op", "concat_strings") ["decider <", spell_id, ">"]
                                                            call %init_peer_id% ("run-console", "print") [title-7, #array-inline-8-0]
                                                            msg_str-7 <- call %init_peer_id% ("json", "stringify") [#array-inline-8-0]
                                                            call %init_peer_id% (spell_id, "list_push_string") ["logs", msg_str-7]
                                                            ap false $result-3
                                                        catch:
                                                            call %init_peer_id% ("errorHandlingSrv", "error") [%last_error%, 8]
                                                catch:
                                                    worker_script <- call %init_peer_id% ("json", "parse") [-result-flat-0.$.[0]]
                                                    worker_config <- call %init_peer_id% ("json", "parse") [-result-flat-0-0.$.[0]]
                                                    worker_ipfs <- call %init_peer_id% ("json", "parse") [-result-flat-0-1.$.[0]]
                                                    WorkerSettings_obj <- call %init_peer_id% ("json", "obj") ["worker_config", worker_config, "worker_ipfs", worker_ipfs, "worker_script", worker_script]
                                                    ap WorkerSettings_obj $settings
                                                    ap true $result-3
                                        canon %init_peer_id% $settings #-settings-fix-0
                                        ap #-settings-fix-0 -settings-flat-0
                                        new $result-3_test:
                                            result-3_incr <- call %init_peer_id% ("math", "add") [0, 1]
                                            fold $result-3 result-3_fold_var:
                                                ap result-3_fold_var $result-3_test
                                                canon %init_peer_id% $result-3_test #result-3_iter_canon
                                                try:
                                                    match #result-3_iter_canon.length result-3_incr:
                                                        null
                                                catch:
                                                    next result-3_fold_var
                                            last:
                                                never
                                            canon %init_peer_id% $result-3_test #result-3_result_canon
                                            ap #result-3_result_canon result-3_gate
                                try:
                                    match result-3_gate.$.[0] false:
                                        ap false $status
                                catch:
                                    try:
                                        worker_id-0 <- call %init_peer_id% ("worker", "create") [deal-0.$.info.deal_id]
                                        call -relay- ("op", "noop") []
                                        try:
                                            WorkerArgs_obj <- call worker_id-0 ("json", "obj") ["deal_id", deal-0.$.info.deal_id, "ipfs", -settings-flat-0.$.[0].worker_ipfs, "worker_def_cid", deal-0.$.info.app_cid]
                                            null
                                            worker_spell_id <- call worker_id-0 ("spell", "install") [-settings-flat-0.$.[0].worker_script, WorkerArgs_obj, -settings-flat-0.$.[0].worker_config]
                                            new $array-inline-9:
                                                ap "created worker for deal" $array-inline-9
                                                ap deal-0.$.info.deal_id $array-inline-9
                                                ap "spell_id" $array-inline-9
                                                ap worker_spell_id $array-inline-9
                                                ap "worker_id" $array-inline-9
                                                ap worker_id-0 $array-inline-9
                                                canon worker_id-0 $array-inline-9 #array-inline-9-0
                                            call -relay- ("op", "noop") []
                                            title-8 <- call %init_peer_id% ("op", "concat_strings") ["decider <", spell_id, ">"]
                                            call %init_peer_id% ("run-console", "print") [title-8, #array-inline-9-0]
                                            msg_str-8 <- call %init_peer_id% ("json", "stringify") [#array-inline-9-0]
                                            call %init_peer_id% (spell_id, "list_push_string") ["logs", msg_str-8]
                                            call -relay- ("op", "noop") []
                                            JoinedDeal_obj <- call worker_id-0 ("json", "obj") ["deal_id", deal-0.$.info.deal_id, "spell_id", worker_spell_id, "worker_id", worker_id-0]
                                            msg <- call worker_id-0 ("json", "stringify") [JoinedDeal_obj]
                                            call worker_id-0 (spell_id, "list_push_string") ["joined_deals", msg]
                                            DealState_obj <- call worker_id-0 ("json", "obj") ["from_block", deal-0.$.block_number]
                                            msg-0 <- call worker_id-0 ("json", "stringify") [DealState_obj]
                                            call worker_id-0 (spell_id, "set_string") [deal-0.$.info.deal_id, msg-0]
                                            ap true $status
                                            call -relay- ("op", "noop") []
                                        catch:
                                            call -relay- ("op", "noop") []
                                            call %init_peer_id% ("errorHandlingSrv", "error") [%last_error%, 9]
                                    catch:
                                        new $array-inline-10:
                                            ap "cannot create worker" $array-inline-10
                                            ap deal-0.$.info.deal_id $array-inline-10
                                            ap %last_error%.$.message $array-inline-10
                                            ap "; skip" $array-inline-10
                                            canon %init_peer_id% $array-inline-10 #array-inline-10-0
                                        title-9 <- call %init_peer_id% ("op", "concat_strings") ["decider <", spell_id, ">"]
                                        call %init_peer_id% ("run-console", "print") [title-9, #array-inline-10-0]
                                        msg_str-9 <- call %init_peer_id% ("json", "stringify") [#array-inline-10-0]
                                        call %init_peer_id% (spell_id, "list_push_string") ["logs", msg_str-9]
                                        ap false $status
                                new $status_test:
                                    status_incr <- call %init_peer_id% ("math", "add") [0, 1]
                                    fold $status status_fold_var:
                                        ap status_fold_var $status_test
                                        canon %init_peer_id% $status_test #status_iter_canon
                                        try:
                                            match #status_iter_canon.length status_incr:
                                                null
                                        catch:
                                            next status_fold_var
                                    last:
                                        never
                                    canon %init_peer_id% $status_test #status_result_canon
                                    ap #status_result_canon status_gate
                            try:
                                match status_gate.$.[0] true:
                                    try:
                                        new $array-inline-11:
                                            ap "joined the deal" $array-inline-11
                                            ap deal-0.$.info.deal_id $array-inline-11
                                            canon %init_peer_id% $array-inline-11 #array-inline-11-0
                                        title-10 <- call %init_peer_id% ("op", "concat_strings") ["decider <", spell_id, ">"]
                                        call %init_peer_id% ("run-console", "print") [title-10, #array-inline-11-0]
                                        msg_str-10 <- call %init_peer_id% ("json", "stringify") [#array-inline-11-0]
                                        call %init_peer_id% (spell_id, "list_push_string") ["logs", msg_str-10]
                                    catch:
                                        call %init_peer_id% ("errorHandlingSrv", "error") [%last_error%, 10]
                            catch:
                                new $array-inline-12:
                                    ap "couldn't join the deal" $array-inline-12
                                    ap deal-0.$.info.deal_id $array-inline-12
                                    canon %init_peer_id% $array-inline-12 #array-inline-12-0
                                title-11 <- call %init_peer_id% ("op", "concat_strings") ["decider <", spell_id, ">"]
                                call %init_peer_id% ("run-console", "print") [title-11, #array-inline-12-0]
                                msg_str-11 <- call %init_peer_id% ("json", "stringify") [#array-inline-12-0]
                                call %init_peer_id% (spell_id, "list_push_string") ["logs", msg_str-11]
                    new_from_block <- call %init_peer_id% ("json", "stringify") [deal-0.$.next_block_number]
                    call %init_peer_id% (spell_id, "set_string") ["from_block", new_from_block]
                    next deal-0
                try:
                    ap result.$.result result_flat
                    ap result_flat result_flat_to_functor
                    ap result_flat_to_functor.length result_flat_length
                    gt <- call %init_peer_id% ("cmp", "gt") [result_flat_length, 1]
                    match gt false:
                        try:
                            null
                            new $need_update:
                                new $block:
                                    new $is_ok-1:
                                        result-8 <- call %init_peer_id% ("fluence_aurora_connector", "latest_block_number") [info.$.api_endpoint]
                                        try:
                                            match result-8.$.success false:
                                                ap false $is_ok-1
                                        catch:
                                            ap result-8.$.result $block
                                            ap true $is_ok-1
                                        canon %init_peer_id% $block #-block-fix-0
                                        ap #-block-fix-0 -block-flat-0
                                        new $is_ok-1_test:
                                            is_ok-1_incr <- call %init_peer_id% ("math", "add") [0, 1]
                                            fold $is_ok-1 is_ok-1_fold_var:
                                                ap is_ok-1_fold_var $is_ok-1_test
                                                canon %init_peer_id% $is_ok-1_test #is_ok-1_iter_canon
                                                try:
                                                    match #is_ok-1_iter_canon.length is_ok-1_incr:
                                                        null
                                                catch:
                                                    next is_ok-1_fold_var
                                            last:
                                                never
                                            canon %init_peer_id% $is_ok-1_test #is_ok-1_result_canon
                                            ap #is_ok-1_result_canon is_ok-1_gate
                                try:
                                    match is_ok-1_gate.$.[0] true:
                                        try:
                                            new $is:
                                                diff <- call %init_peer_id% ("fluence_aurora_connector", "blocks_diff") [result.$.to_block, -block-flat-0.$.[0]]
                                                try:
                                                    match diff 0:
                                                        ap false $is
                                                catch:
                                                    ap true $is
                                                new $is_test:
                                                    is_incr <- call %init_peer_id% ("math", "add") [0, 1]
                                                    fold $is is_fold_var:
                                                        ap is_fold_var $is_test
                                                        canon %init_peer_id% $is_test #is_iter_canon
                                                        try:
                                                            match #is_iter_canon.length is_incr:
                                                                null
                                                        catch:
                                                            next is_fold_var
                                                    last:
                                                        never
                                                    canon %init_peer_id% $is_test #is_result_canon
                                                    ap #is_result_canon is_gate
                                            ap is_gate.$.[0] $need_update
                                        catch:
                                            call %init_peer_id% ("errorHandlingSrv", "error") [%last_error%, 11]
                                catch:
                                    ap false $need_update
                                new $need_update_test:
                                    need_update_incr <- call %init_peer_id% ("math", "add") [0, 1]
                                    fold $need_update need_update_fold_var:
                                        ap need_update_fold_var $need_update_test
                                        canon %init_peer_id% $need_update_test #need_update_iter_canon
                                        try:
                                            match #need_update_iter_canon.length need_update_incr:
                                                null
                                        catch:
                                            next need_update_fold_var
                                    last:
                                        never
                                    canon %init_peer_id% $need_update_test #need_update_result_canon
                                    ap #need_update_result_canon need_update_gate
                            try:
                                match need_update_gate.$.[0] true:
                                    try:
                                        new $array-inline-13:
                                            ap "updating outdated from_block: [previous from_block, new_from_block]" $array-inline-13
                                            ap from_block $array-inline-13
                                            ap result.$.to_block $array-inline-13
                                            canon %init_peer_id% $array-inline-13 #array-inline-13-0
                                        title-12 <- call %init_peer_id% ("op", "concat_strings") ["decider <", spell_id, ">"]
                                        call %init_peer_id% ("run-console", "print") [title-12, #array-inline-13-0]
                                        msg_str-12 <- call %init_peer_id% ("json", "stringify") [#array-inline-13-0]
                                        call %init_peer_id% (spell_id, "list_push_string") ["logs", msg_str-12]
                                        to_block_str <- call %init_peer_id% ("json", "stringify") [result.$.to_block]
                                        call %init_peer_id% (spell_id, "set_string") ["from_block", to_block_str]
                                    catch:
                                        call %init_peer_id% ("errorHandlingSrv", "error") [%last_error%, 12]
                            catch:
                                call %init_peer_id% ("op", "noop") []
                        catch:
                            call %init_peer_id% ("errorHandlingSrv", "error") [%last_error%, 13]
                catch:
                    call %init_peer_id% ("op", "noop") []
    catch:
        call %init_peer_id% ("op", "noop") []
    new $deals_update:
        new $block-0:
            new $is_ok-3:
                result-9 <- call %init_peer_id% ("fluence_aurora_connector", "latest_block_number") [info.$.api_endpoint]
                try:
                    match result-9.$.success false:
                        ap false $is_ok-3
                catch:
                    ap result-9.$.result $block-0
                    ap true $is_ok-3
                canon %init_peer_id% $block-0 #-block-fix-0-0
                ap #-block-fix-0-0 -block-flat-0-0
                new $is_ok-3_test:
                    is_ok-3_incr <- call %init_peer_id% ("math", "add") [0, 1]
                    fold $is_ok-3 is_ok-3_fold_var:
                        ap is_ok-3_fold_var $is_ok-3_test
                        canon %init_peer_id% $is_ok-3_test #is_ok-3_iter_canon
                        try:
                            match #is_ok-3_iter_canon.length is_ok-3_incr:
                                null
                        catch:
                            next is_ok-3_fold_var
                    last:
                        never
                    canon %init_peer_id% $is_ok-3_test #is_ok-3_result_canon
                    ap #is_ok-3_result_canon is_ok-3_gate
        list <- call %init_peer_id% (spell_id, "list_get_strings") ["joined_deals"]
        try:
            match list.$.success false:
                try:
                    new $array-inline-14:
                        ap "can't restrive joined deals" $array-inline-14
                        ap list.$.error $array-inline-14
                        canon %init_peer_id% $array-inline-14 #array-inline-14-0
                    title-13 <- call %init_peer_id% ("op", "concat_strings") ["decider <", spell_id, ">"]
                    call %init_peer_id% ("run-console", "print") [title-13, #array-inline-14-0]
                    msg_str-13 <- call %init_peer_id% ("json", "stringify") [#array-inline-14-0]
                    call %init_peer_id% (spell_id, "list_push_string") ["logs", msg_str-13]
                catch:
                    call %init_peer_id% ("errorHandlingSrv", "error") [%last_error%, 14]
        catch:
            fold list.$.strings joined_deal_str-0:
                joined_deal <- call %init_peer_id% ("json", "parse") [joined_deal_str-0]
                new $status-3:
                    new $result-10:
                        value-2 <- call %init_peer_id% (spell_id, "get_string") [joined_deal.$.deal_id]
                        try:
                            match value-2.$.success false:
                                ap false $status-3
                        catch:
                            try:
                                match value-2.$.absent true:
                                    ap false $status-3
                            catch:
                                ap value-2.$.str $result-10
                                ap true $status-3
                        canon %init_peer_id% $result-10 #-result-fix-0-2
                        ap #-result-fix-0-2 -result-flat-0-2
                        new $status-3_test:
                            status-3_incr <- call %init_peer_id% ("math", "add") [0, 1]
                            fold $status-3 status-3_fold_var:
                                ap status-3_fold_var $status-3_test
                                canon %init_peer_id% $status-3_test #status-3_iter_canon
                                try:
                                    match #status-3_iter_canon.length status-3_incr:
                                        null
                                catch:
                                    next status-3_fold_var
                            last:
                                never
                            canon %init_peer_id% $status-3_test #status-3_result_canon
                            ap #status-3_result_canon status-3_gate
                try:
                    match status-3_gate.$.[0] false:
                        try:
                            new $array-inline-15:
                                ap "can't find state of the deal" $array-inline-15
                                ap joined_deal.$.deal_id $array-inline-15
                                ap "; broken invariant, check poll_new_deals" $array-inline-15
                                canon %init_peer_id% $array-inline-15 #array-inline-15-0
                            title-14 <- call %init_peer_id% ("op", "concat_strings") ["decider <", spell_id, ">"]
                            call %init_peer_id% ("run-console", "print") [title-14, #array-inline-15-0]
                            msg_str-14 <- call %init_peer_id% ("json", "stringify") [#array-inline-15-0]
                            call %init_peer_id% (spell_id, "list_push_string") ["logs", msg_str-14]
                        catch:
                            call %init_peer_id% ("errorHandlingSrv", "error") [%last_error%, 15]
                catch:
                    deal_state <- call %init_peer_id% ("json", "parse") [-result-flat-0-2.$.[0]]
                    DealInfo_obj <- call %init_peer_id% ("json", "obj") ["deal_id", joined_deal.$.deal_id, "worker_id", joined_deal.$.worker_id]
                    DealUpdate_obj <- call %init_peer_id% ("json", "obj") ["deal_info", DealInfo_obj, "from_block", deal_state.$.from_block]
                    ap DealUpdate_obj $deals_update
                next joined_deal_str-0
            canon %init_peer_id% $deals_update #deals_update_canon
            updated_deals <- call %init_peer_id% ("fluence_aurora_connector", "poll_deals_latest_update_batch") [info.$.api_endpoint, #deals_update_canon]
            try:
                match updated_deals.$.success false:
                    try:
                        new $array-inline-16:
                            ap "can't download deals updates" $array-inline-16
                            ap updated_deals.$.error.[0] $array-inline-16
                            canon %init_peer_id% $array-inline-16 #array-inline-16-0
                        title-15 <- call %init_peer_id% ("op", "concat_strings") ["decider <", spell_id, ">"]
                        call %init_peer_id% ("run-console", "print") [title-15, #array-inline-16-0]
                        msg_str-15 <- call %init_peer_id% ("json", "stringify") [#array-inline-16-0]
                        call %init_peer_id% (spell_id, "list_push_string") ["logs", msg_str-15]
                    catch:
                        call %init_peer_id% ("errorHandlingSrv", "error") [%last_error%, 16]
            catch:
                fold updated_deals.$.result updated_deal-0:
                    try:
                        match updated_deal-0.$.success false:
                            try:
                                new $array-inline-17:
                                    ap "error retrieving deal update" $array-inline-17
                                    ap updated_deal-0.$.deal_info.deal_id $array-inline-17
                                    ap updated_deal-0.$.error $array-inline-17
                                    canon %init_peer_id% $array-inline-17 #array-inline-17-0
                                title-16 <- call %init_peer_id% ("op", "concat_strings") ["decider <", spell_id, ">"]
                                call %init_peer_id% ("run-console", "print") [title-16, #array-inline-17-0]
                                msg_str-16 <- call %init_peer_id% ("json", "stringify") [#array-inline-17-0]
                                call %init_peer_id% (spell_id, "list_push_string") ["logs", msg_str-16]
                                try:
                                    match is_ok-3_gate.$.[0] true:
                                        try:
                                            try:
                                                null
                                                new $is-0:
                                                    diff-0 <- call %init_peer_id% ("fluence_aurora_connector", "blocks_diff") [updated_deal-0.$.to_block, -block-flat-0-0.$.[0]]
                                                    try:
                                                        match diff-0 0:
                                                            ap false $is-0
                                                    catch:
                                                        ap true $is-0
                                                    new $is-0_test:
                                                        is-0_incr <- call %init_peer_id% ("math", "add") [0, 1]
                                                        fold $is-0 is-0_fold_var:
                                                            ap is-0_fold_var $is-0_test
                                                            canon %init_peer_id% $is-0_test #is-0_iter_canon
                                                            try:
                                                                match #is-0_iter_canon.length is-0_incr:
                                                                    null
                                                            catch:
                                                                next is-0_fold_var
                                                        last:
                                                            never
                                                        canon %init_peer_id% $is-0_test #is-0_result_canon
                                                        ap #is-0_result_canon is-0_gate
                                                match is-0_gate.$.[0] true:
                                                    try:
                                                        new $array-inline-18:
                                                            ap "update from_block: [new from_block, latest_block]" $array-inline-18
                                                            ap updated_deal-0.$.to_block $array-inline-18
                                                            ap -block-flat-0-0 $array-inline-18
                                                            canon %init_peer_id% $array-inline-18 #array-inline-18-0
                                                        title-17 <- call %init_peer_id% ("op", "concat_strings") ["decider <", spell_id, ">"]
                                                        call %init_peer_id% ("run-console", "print") [title-17, #array-inline-18-0]
                                                        msg_str-17 <- call %init_peer_id% ("json", "stringify") [#array-inline-18-0]
                                                        call %init_peer_id% (spell_id, "list_push_string") ["logs", msg_str-17]
                                                        DealState_obj-0 <- call %init_peer_id% ("json", "obj") ["from_block", updated_deal-0.$.to_block]
                                                        msg-19 <- call %init_peer_id% ("json", "stringify") [DealState_obj-0]
                                                        call %init_peer_id% (spell_id, "set_string") [updated_deal-0.$.deal_info.deal_id, msg-19]
                                                    catch:
                                                        call %init_peer_id% ("errorHandlingSrv", "error") [%last_error%, 17]
                                            catch:
                                                call %init_peer_id% ("op", "noop") []
                                        catch:
                                            call %init_peer_id% ("errorHandlingSrv", "error") [%last_error%, 18]
                                catch:
                                    call %init_peer_id% ("op", "noop") []
                            catch:
                                call %init_peer_id% ("errorHandlingSrv", "error") [%last_error%, 19]
                    catch:
                        new $array-inline-19:
                            ap "found update for deal" $array-inline-19
                            ap updated_deal-0.$.deal_info.deal_id $array-inline-19
                            ap "from block" $array-inline-19
                            ap updated_deal-0.$.result.[0].block_number $array-inline-19
                            canon %init_peer_id% $array-inline-19 #array-inline-19-0
                        title-18 <- call %init_peer_id% ("op", "concat_strings") ["decider <", spell_id, ">"]
                        call %init_peer_id% ("run-console", "print") [title-18, #array-inline-19-0]
                        msg_str-18 <- call %init_peer_id% ("json", "stringify") [#array-inline-19-0]
                        call %init_peer_id% (spell_id, "list_push_string") ["logs", msg_str-18]
                        DealState_obj-1 <- call %init_peer_id% ("json", "obj") ["from_block", updated_deal-0.$.result.[0].next_block_number]
                        msg-22 <- call %init_peer_id% ("json", "stringify") [DealState_obj-1]
                        call %init_peer_id% (spell_id, "set_string") [updated_deal-0.$.deal_info.deal_id, msg-22]
                        new $array-inline-20:
                            ap "sending the latest update to the worker" $array-inline-20
                            ap updated_deal-0.$.deal_info $array-inline-20
                            canon %init_peer_id% $array-inline-20 #array-inline-20-0
                        title-19 <- call %init_peer_id% ("op", "concat_strings") ["decider <", spell_id, ">"]
                        call %init_peer_id% ("run-console", "print") [title-19, #array-inline-20-0]
                        msg_str-19 <- call %init_peer_id% ("json", "stringify") [#array-inline-20-0]
                        call %init_peer_id% (spell_id, "list_push_string") ["logs", msg_str-19]
                        call -relay- ("op", "noop") []
                        try:
                            app_cid <- call updated_deal-0.$.deal_info.worker_id ("json", "stringify") [updated_deal-0.$.result.[0].info.app_cid]
                            call updated_deal-0.$.deal_info.worker_id ("worker-spell", "set_string") ["worker_def_cid", app_cid]
                            call -relay- ("op", "noop") []
                        catch:
                            call -relay- ("op", "noop") []
                            call %init_peer_id% ("errorHandlingSrv", "error") [%last_error%, 20]
                    next updated_deal-0
catch:
    call %init_peer_id% ("errorHandlingSrv", "error") [%last_error%, 21]

