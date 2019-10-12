import CodeMirror from 'codemirror'

CodeMirror.defineMode("perl6", () => {
    //   null - magic touch
    //   1 - keyword
    //   2 - def
    //   3 - type
    //   4 - operator
    //   5 - variable-2 (predefined)
    //   [x,y] - x=1,2,3; y=must be defined if x{...}
    const PERL6 = {
        // Perl operators
        temp: 4,
        let: 4,
        but: 4,
        does: 4,
        leg: 4,
        unicmp: 4,
        cmp: 4,
        coll: 4,
        eq: 4,
        ne: 4,
        lt: 4,
        le: 4,
        gt: 4,
        ge: 4,
        eqv: 4,
        '(elem)': 4,
        '(cont)': 4,
        min: 4,
        max: 4,
        so: 4,
        not: 4,
        Z: 4,
        minmax: 4,
        X: 4,
        Xeqv: 4,
        and: 4,
        andthen: 4,
        notandthen: 4,
        or: 4,
        xor: 4,
        orelse: 4,
        // Keywords
        BEGIN: [1, 1],
        CATCH: [1, 1],
        CHECK: [1, 1],
        CONTROL: [1, 1],
        END: [1, 1],
        ENTER: [1, 1],
        FIRST: [1, 1],
        INIT: [1, 1],
        KEEP: [1, 1],
        LAST: [1, 1],
        LEAVE: [1, 1],
        NEXT: [1, 1],
        POST: [1, 1],
        PRE: [1, 1],
        START: [1, 1],
        TEMP: [1, 1],
        UNDO: [1, 1],
        assoc: [1, 1],
        async: [1, 1],
        binary: [1, 1],
        break: [1, 1],
        cached: [1, 1],
        category: [1, 1],
        class: [1, 1],
        constant: [1, 1],
        contend: [1, 1],
        continue: [1, 1],
        copy: [1, 1],
        deep: [1, 1],
        default: [1, 1],
        defequiv: [1, 1],
        defer: [1, 1],
        die: [1, 1],
        do: [1, 1],
        else: [1, 1],
        elsif: [1, 1],
        enum: [1, 1],
        equiv: [1, 1],
        exit: [1, 1],
        export: [1, 1],
        fail: [1, 1],
        for: [1, 1],
        gather: [1, 1],
        given: [1, 1],
        goto: [1, 1],
        grammar: [1, 1],
        has: [1, 1],
        if: [1, 1],
        inline: [1, 1],
        irs: [1, 1],
        last: [1, 1],
        leave: [1, 1],
        let: [1, 1],
        lift: [1, 1],
        loop: [1, 1],
        looser: [1, 1],
        macro: [1, 1],
        make: [1, 1],
        maybe: [1, 1],
        method: [1, 1],
        module: [1, 1],
        multi: [1, 1],
        my: 2,
        next: [1, 1],
        ofs: [1, 1],
        only: [1, 1],
        ors: [1, 1],
        our: 2,
        package: [1, 1],
        parsed: [1, 1],
        prec: [1, 1],
        proto: [1, 1],
        readonly: [1, 1],
        react: [1, 1],
        redo: [1, 1],
        ref: [1, 1],
        regex: [1, 1],
        reparsed: [1, 1],
        repeat: [1, 1],
        require: [1, 1],
        required: [1, 1],
        return: [1, 1],
        role: [1, 1],
        rule: [1, 1],
        rw: [1, 1],
        slang: [1, 1],
        state: 2,
        sub: [1, 1],
        submethod: [1, 1],
        subset: [1, 1],
        take: [1, 1],
        temp: [1, 1],
        tighter: [1, 1],
        token: [1, 1],
        try: [1, 1],
        unary: [1, 1],
        unless: [1, 1],
        until: [1, 1],
        use: [1, 1],
        warn: [1, 1],
        whanever: [1, 1],
        when: [1, 1],
        while: [1, 1],
        will: [1, 1],
        with: [1, 1],
        // Perl functions
        abs: 1,
        absolute: 1,
        accept: 1,
        ACCEPTS: 1,
        accessed: 1,
        acos: 1,
        acosec: 1,
        acosech: 1,
        acosh: 1,
        acotan: 1,
        acotanh: 1,
        acquire: 1,
        act: 1,
        action: 1,
        actions: 1,
        add: 1,
        addendum: 1,
        adverb: 1,
        after: 1,
        all: 1,
        allocate: 1,
        allof: 1,
        allowed: 1,
        "alternative-names": 1,
        and: 1,
        andthen: 1,
        annotations: 1,
        antipair: 1,
        antipairs: 1,
        any: 1,
        anyof: 1,
        api: 1,
        append: 1,
        arch: 1,
        archetypes: 1,
        archname: 1,
        args: 1,
        "ARGS-TO-CAPTURE": 1,
        arity: 1,
        Array: 3,
        asec: 1,
        asech: 1,
        asin: 1,
        asinh: 1,
        "ASSIGN-KEY": 1,
        "ASSIGN-POS": 1,
        assuming: 1,
        ast: 1,
        at: 1,
        "AT-KEY": 1,
        "AT-POS": 1,
        atan: 1,
        atanh: 1,
        "atomic-assign": 1,
        "atomic-dec-fetch": 1,
        "atomic-fetch": 1,
        "atomic-fetch-add": 1,
        "atomic-fetch-dec": 1,
        "atomic-fetch-inc": 1,
        "atomic-fetch-sub": 1,
        "atomic-inc-fetch": 1,
        attributes: 1,
        auth: 1,
        await: 1,
        backtrace: 1,
        Bag: 3,
        bag: 1,
        Baggy: 3,
        BagHash: 1,
        "bail-out": 1,
        base: 1,
        "base-repeating": 1,
        basename: 1,
        batch: 1,
        before: 1,
        "BIND-KEY": 1,
        "BIND-POS": 1,
        "bind-stderr": 1,
        "bind-stdin": 1,
        "bind-stdout": 1,
        "bind-udp": 1,
        bits: 1,
        bless: 1,
        block: 1,
        Bool: 3,
        "bool-only": 1,
        bounds: 1,
        break: 1,
        Bridge: 3,
        broken: 1,
        BUILD: 1,
        "build-date": 1,
        but: 1,
        bytes: 1,
        cache: 1,
        "CALL-ME": 1,
        callframe: 1,
        "calling-package": 1,
        callsame: 1,
        callwith: 1,
        can: 1,
        "can-ok": 1,
        cancel: 1,
        candidates: 1,
        cando: 1,
        canonpath: 1,
        caps: 1,
        caption: 1,
        Capture: 3,
        capture: 1,
        cas: 1,
        catdir: 1,
        categorize: 1,
        "categorize-list": 1,
        catfile: 1,
        catpath: 1,
        cause: 1,
        ceiling: 1,
        cglobal: 1,
        changed: 1,
        Channel: 3,
        channel: 1,
        chars: 1,
        chdir: 1,
        child: 1,
        "child-name": 1,
        "child-typename": 1,
        chmod: 1,
        chomp: 1,
        chop: 1,
        chr: 1,
        chrs: 1,
        chunks: 1,
        cis: 1,
        classify: 1,
        "classify-list": 1,
        cleanup: 1,
        clone: 1,
        close: 1,
        "close-stdin": 1,
        closed: 1,
        cmp: 1,
        "cmp-ok": 1,
        code: 1,
        codename: 1,
        codes: 1,
        coll: 1,
        collate: 1,
        column: 1,
        comb: 1,
        combinations: 1,
        command: 1,
        comment: 1,
        compiler: 1,
        Complex: 3,
        composalizer: 1,
        compose: 1,
        composer: 1,
        concise: 1,
        condition: 1,
        config: 1,
        conj: 1,
        connect: 1,
        constant: 1,
        constraints: 1,
        construct: 1,
        contains: 1,
        content: 1,
        contents: 1,
        Cool: 1,
        copy: 1,
        cos: 1,
        cosec: 1,
        cosech: 1,
        cosh: 1,
        cotan: 1,
        cotanh: 1,
        count: 1,
        "count-only": 1,
        "cpu-cores": 1,
        "cpu-usage": 1,
        CREATE: 1,
        cross: 1,
        cue: 1,
        curdir: 1,
        curupdir: 1,
        d: 1,
        Date: 3,
        DateTime: 1,
        day: 1,
        "day-of-month": 1,
        "day-of-week": 1,
        "day-of-year": 1,
        daycount: 1,
        "days-in-month": 1,
        debug: 1,
        declaration: 1,
        decode: 1,
        decoder: 1,
        deepmap: 1,
        default: 1,
        defined: 1,
        delayed: 1,
        "DELETE-KEY": 1,
        "DELETE-POS": 1,
        denominator: 1,
        desc: 1,
        DESTROY: 1,
        destroyers: 1,
        devnull: 1,
        diag: 1,
        "did-you-mean": 1,
        die: 1,
        "dies-ok": 1,
        dir: 1,
        "dir-sep": 1,
        dirname: 1,
        distribution: 1,
        DISTROnames: 1,
        div: 1,
        do: 1,
        does: 1,
        "does-ok": 1,
        done: 1,
        "done-testing": 1,
        duckmap: 1,
        dynamic: 1,
        e: 5,
        eager: 1,
        earlier: 1,
        elems: 1,
        emit: 1,
        enclosing: 1,
        encode: 1,
        encoder: 1,
        encoding: 1,
        end: 1,
        endian: 1,
        "ends-with": 1,
        enums: 1,
        EOF: 1,
        eof: 1,
        eq: 1,
        eqv: 1,
        EVAL: 1,
        "eval-dies-ok": 1,
        "eval-lives-ok": 1,
        EVALFILE: 1,
        exception: 1,
        "excludes-max": 1,
        "excludes-min": 1,
        "EXISTS-KEY": 1,
        "EXISTS-POS": 1,
        exit: 1,
        exitcode: 1,
        exp: 1,
        expected: 1,
        "explicitly-manage": 1,
        expmod: 1,
        extension: 1,
        f: 1,
        fail: 1,
        "fails-like": 1,
        FALLBACK: 1,
        fc: 1,
        feature: 1,
        ff: 1,
        fff: 1,
        file: 1,
        filename: 1,
        files: 1,
        find: 1,
        finish: 1,
        first: 1,
        flat: 1,
        flatmap: 1,
        flip: 1,
        floor: 1,
        flunk: 1,
        flush: 1,
        fmt: 1,
        format: 1,
        formatter: 1,
        "free-memory": 1,
        freeze: 1,
        from: 1,
        "from-list": 1,
        "from-loop": 1,
        "from-posix": 1,
        "from-slurpy": 1,
        full: 1,
        "full-barrier": 1,
        gcd: 1,
        ge: 1,
        "GENERATE-USAGE": 1,
        get: 1,
        getc: 1,
        gist: 1,
        got: 1,
        grab: 1,
        grabpairs: 1,
        grep: 1,
        gt: 1,
        handle: 1,
        handled: 1,
        handles: 1,
        hardware: 1,
        Hash: 3,
        hash: 1,
        head: 1,
        headers: 1,
        "hh-mm-ss": 1,
        hidden: 1,
        hides: 1,
        hostname: 1,
        hour: 1,
        how: 1,
        hyper: 1,
        i: 5,
        id: 1,
        illegal: 1,
        im: 1,
        in: 1,
        "in-timezone": 1,
        indent: 1,
        index: 1,
        indices: 1,
        indir: 1,
        infinite: 1,
        infix: 1,
        install: 1,
        Instant: 3,
        instead: 1,
        Int: 3,
        "int-bounds": 1,
        interval: 1,
        "invalid-str": 1,
        invert: 1,
        invocant: 1,
        IO: 1,
        is: 1,
        "is-absolute": 1,
        "is-approx": 1,
        "is-approx-calculate": 1,
        "is-deeply": 1,
        "is-hidden": 1,
        "is-initial-thread": 1,
        "is-int": 1,
        "is-lazy": 1,
        "is-leap-year": 1,
        "is-prime": 1,
        "is-relative": 1,
        "is-routine": 1,
        "is-setting": 1,
        "is-win": 1,
        isa: 1,
        "isa-ok": 1,
        isNaN: 1,
        isnt: 1,
        item: 1,
        iterator: 1,
        join: 1,
        keep: 1,
        kept: 1,
        KERNELnames: 1,
        key: 1,
        keyof: 1,
        keys: 1,
        kill: 1,
        kv: 1,
        kxxv: 1,
        l: 1,
        lang: 1,
        last: 1,
        lastcall: 1,
        later: 1,
        lazy: 1,
        lc: 1,
        lcm: 1,
        le: 1,
        leading: 1,
        leg: 1,
        let: 1,
        level: 1,
        like: 1,
        line: 1,
        lines: 1,
        link: 1,
        List: 3,
        list: 1,
        listen: 1,
        live: 1,
        "lives-ok": 1,
        load: 1,
        loaded: 1,
        local: 1,
        lock: 1,
        log: 1,
        lookup: 1,
        lsb: 1,
        lt: 1,
        made: 1,
        MAIN: 1,
        make: 1,
        Map: 3,
        map: 1,
        match: 1,
        max: 1,
        maxpairs: 1,
        merge: 1,
        message: 1,
        meta: 1,
        method: 1,
        methods: 1,
        migrate: 1,
        min: 1,
        minmax: 1,
        minpairs: 1,
        minute: 1,
        misplaced: 1,
        Mix: 3,
        mix: 1,
        MixHash: 1,
        Mixy: 3,
        mkdir: 1,
        mod: 1,
        mode: 1,
        modified: 1,
        month: 1,
        move: 1,
        mro: 1,
        msb: 1,
        multi: 1,
        multiness: 1,
        my: 1,
        name: 1,
        named: 1,
        narrow: 1,
        "native-descriptor": 1,
        nativecast: 1,
        nativesizeof: 1,
        ne: 1,
        need: 1,
        new: 1,
        "new-from-daycount": 1,
        "new-from-pairs": 1,
        next: 1,
        "next-handle": 1,
        "next-interesting-index": 1,
        nextcallee: 1,
        nextsame: 1,
        nextwith: 1,
        NFC: 1,
        NFD: 1,
        NFKC: 1,
        NFKD: 1,
        nice: 1,
        Nil: 3,
        "nl-in": 1,
        "nl-out": 1,
        nodemap: 1,
        nok: 1,
        none: 1,
        norm: 1,
        not: 1,
        notandthen: 1,
        note: 1,
        now: 1,
        nude: 1,
        Num: 3,
        numerator: 1,
        Numeric: 3,
        of: 1,
        offset: 1,
        "offset-in-hours": 1,
        "offset-in-minutes": 1,
        ok: 1,
        old: 1,
        "on-close": 1,
        "on-switch": 1,
        one: 1,
        open: 1,
        opened: 1,
        operation: 1,
        optional: 1,
        or: 1,
        ord: 1,
        ords: 1,
        orelse: 1,
        orig: 1,
        "os-error": 1,
        osname: 1,
        "out-buffer": 1,
        "outer-caller-idx": 1,
        pack: 1,
        package: 1,
        "package-kind": 1,
        "package-name": 1,
        packages: 1,
        Pair: 3,
        pair: 1,
        pairs: 1,
        pairup: 1,
        parameter: 1,
        params: 1,
        parent: 1,
        "parent-name": 1,
        parents: 1,
        parse: 1,
        "parse-base": 1,
        "parse-names": 1,
        parsefile: 1,
        parts: 1,
        pass: 1,
        path: 1,
        "path-sep": 1,
        payload: 1,
        "peer-host": 1,
        "peer-port": 1,
        periods: 1,
        perl: 1,
        permutations: 1,
        phaser: 1,
        pi: 5,
        pick: 1,
        pickpairs: 1,
        pid: 1,
        placeholder: 1,
        plan: 1,
        plus: 1,
        polar: 1,
        poll: 1,
        polymod: 1,
        pop: 1,
        pos: 1,
        positional: 1,
        posix: 1,
        postfix: 1,
        postmatch: 1,
        "precomp-ext": 1,
        "precomp-target": 1,
        precompiled: 1,
        pred: 1,
        prefix: 1,
        prematch: 1,
        prepend: 1,
        primary: 1,
        print: 1,
        "print-nl": 1,
        "print-to": 1,
        printf: 1,
        private: 1,
        proc: 1,
        produce: 1,
        Promise: 3,
        promise: 1,
        prompt: 1,
        protect: 1,
        "protect-or-queue-on-recursion": 1,
        "pull-one": 1,
        push: 1,
        "push-all": 1,
        "push-at-least": 1,
        "push-exactly": 1,
        "push-until-lazy": 1,
        put: 1,
        "qualifier-type": 1,
        quaternary: 1,
        quit: 1,
        r: 1,
        race: 1,
        radix: 1,
        rand: 1,
        Range: 3,
        range: 1,
        Rat: 3,
        raw: 1,
        re: 1,
        READ: 1,
        read: 1,
        "read-bits": 1,
        "read-ubits": 1,
        readchars: 1,
        readonly: 1,
        ready: 1,
        Real: 3,
        reallocate: 1,
        reals: 1,
        reason: 1,
        rebless: 1,
        receive: 1,
        recv: 1,
        redispatcher: 1,
        redo: 1,
        reduce: 1,
        relative: 1,
        release: 1,
        rename: 1,
        repeated: 1,
        replacement: 1,
        repo: 1,
        "repo-id": 1,
        report: 1,
        required: 1,
        reserved: 1,
        resolve: 1,
        restore: 1,
        result: 1,
        resume: 1,
        rethrow: 1,
        return: 1,
        "return-rw": 1,
        returns: 1,
        reverse: 1,
        right: 1,
        rindex: 1,
        rmdir: 1,
        role: 1,
        rolish: 1,
        roll: 1,
        rootdir: 1,
        roots: 1,
        rotate: 1,
        rotor: 1,
        round: 1,
        roundrobin: 1,
        "routine-type": 1,
        run: 1,
        "RUN-MAIN": 1,
        rw: 1,
        rwx: 1,
        s: 1,
        samecase: 1,
        samemark: 1,
        samewith: 1,
        say: 1,
        "schedule-on": 1,
        scheduler: 1,
        scope: 1,
        sec: 1,
        sech: 1,
        second: 1,
        secondary: 1,
        seek: 1,
        self: 5,
        send: 1,
        serial: 1,
        Set: 3,
        set: 1,
        "set-instruments": 1,
        SetHash: 1,
        Setty: 3,
        shape: 1,
        share: 1,
        shell: 1,
        shift: 1,
        "short-id": 1,
        "short-name": 1,
        sibling: 1,
        sigil: 1,
        sign: 1,
        signal: 1,
        signals: 1,
        signature: 1,
        sin: 1,
        sinh: 1,
        sink: 1,
        "sink-all": 1,
        skip: 1,
        "skip-at-least": 1,
        "skip-at-least-pull-one": 1,
        "skip-one": 1,
        "skip-rest": 1,
        sleep: 1,
        "sleep-timer": 1,
        "sleep-until": 1,
        Slip: 3,
        slip: 1,
        slurp: 1,
        "slurp-rest": 1,
        slurpy: 1,
        snap: 1,
        snapper: 1,
        so: 1,
        "socket-host": 1,
        "socket-port": 1,
        sort: 1,
        source: 1,
        "source-package": 1,
        spawn: 1,
        SPEC: 1,
        splice: 1,
        split: 1,
        splitdir: 1,
        splitpath: 1,
        sprintf: 1,
        spurt: 1,
        sqrt: 1,
        squish: 1,
        srand: 1,
        stable: 1,
        start: 1,
        started: 1,
        "starts-with": 1,
        status: 1,
        stderr: 1,
        stdout: 1,
        STORE: 1,
        Str: 3,
        Stringy: 3,
        subbuf: 1,
        "subbuf-rw": 1,
        subname: 1,
        subparse: 1,
        subst: 1,
        "subst-mutate": 1,
        substr: 1,
        "substr-eq": 1,
        "substr-rw": 1,
        subtest: 1,
        succ: 1,
        sum: 1,
        summary: 1,
        Supply: 3,
        symbol: 1,
        symlink: 1,
        T: 1,
        t: 1,
        tail: 1,
        take: 1,
        "take-rw": 1,
        tan: 1,
        tanh: 1,
        tap: 1,
        target: 1,
        "target-name": 1,
        tau: 5,
        tc: 1,
        tclc: 1,
        tell: 1,
        temp: 1,
        term: 1,
        tertiary: 1,
        then: 1,
        throttle: 1,
        throw: 1,
        "throws-like": 1,
        time: 1,
        timezone: 1,
        tmpdir: 1,
        to: 1,
        "to-posix": 1,
        today: 1,
        todo: 1,
        toggle: 1,
        total: 1,
        "total-memory": 1,
        trailing: 1,
        trans: 1,
        tree: 1,
        trim: 1,
        "trim-leading": 1,
        "trim-trailing": 1,
        truncate: 1,
        "truncated-to": 1,
        trusts: 1,
        trying: 1,
        twigil: 1,
        type: 1,
        typename: 1,
        uc: 1,
        udp: 1,
        undefine: 1,
        unicmp: 1,
        unimatch: 1,
        uniname: 1,
        uninames: 1,
        uninstall: 1,
        uniparse: 1,
        uniprop: 1,
        uniprops: 1,
        unique: 1,
        unival: 1,
        univals: 1,
        unlike: 1,
        unlink: 1,
        unlock: 1,
        unpack: 1,
        unpolar: 1,
        unshift: 1,
        unwrap: 1,
        updir: 1,
        USAGE: 1,
        "use-ok": 1,
        utc: 1,
        val: 1,
        value: 1,
        values: 1,
        variable: 1,
        ver: 1,
        "verbose-config": 1,
        version: 1,
        VMnames: 1,
        volume: 1,
        vow: 1,
        w: 1,
        wait: 1,
        warn: 1,
        watch: 1,
        "watch-path": 1,
        week: 1,
        "week-number": 1,
        "week-year": 1,
        "weekday-of-month": 1,
        what: 1,
        when: 1,
        WHERE: 1,
        WHEREFORE: 1,
        WHICH: 1,
        "whole-second": 1,
        WHY: 1,
        why: 1,
        "with-lock-hidden-from-recursion-check": 1,
        wordcase: 1,
        words: 1,
        workaround: 1,
        wrap: 1,
        WRITE: 1,
        write: 1,
        "write-bits": 1,
        "write-to": 1,
        "write-ubits": 1,
        X: 1,
        x: 1,
        xor: 1,
        xx: 1,
        yada: 1,
        year: 1,
        yield: 1,
        "yyyy-mm-dd": 1,
        Z: 1,
        z: 1,
        zip: 1,
        "zip-latest": 1,
        π: 5,
        τ: 5,
        𝑒: 5,
        '∅': 5,
        '$_': 5,
        '$!': 5,
        '$/': 5,
    }

    var RXstyle = "string-2"
    var RXmodifiers = /[goseximacplud]/               // NOTE: "m", "s", "y" and "tr" need to correct real modifiers for each regexp type

    function tokenChain(stream, state, chain, style, tail) {     // NOTE: chain.length > 2 is not working now (it's for s[...][...]geos;)
        state.chain = null                                //                                                          12   3tail
        state.style = null
        state.tail = null
        state.tokenize = function (stream, state) {
            var e = false, c, i = 0
            while (c = stream.next()) {
                if (c === chain[i] && !e) {
                    if (chain[++i] !== undefined) {
                        state.chain = chain[i]
                        state.style = style
                        state.tail = tail
                    }
                    else if (tail)
                        stream.eatWhile(tail)
                    state.tokenize = tokenPerl
                    return style
                }
                e = !e && c == "\\"
            }
            return style
        }
        return state.tokenize(stream, state)
    }

    function tokenSOMETHING(stream, state, string) {
        state.tokenize = function (stream, state) {
            if (stream.string == string)
                state.tokenize = tokenPerl
            stream.skipToEnd()
            return "string"
        }
        return state.tokenize(stream, state)
    }

    function tokenPerl(stream, state) {
        if (stream.eatSpace())
            return null
        if (state.chain)
            return tokenChain(stream, state, state.chain, state.style, state.tail)
        if (stream.match(/^\-?[\d\.]/, false))
            if (stream.match(/^(\-?(\d*\.\d+(e[+-]?\d+)?|\d+\.\d*)|0x[\da-fA-F]+|0b[01]+|\d+(e[+-]?\d+)?)/))
                return 'number'
        if (stream.match(/^<<(?=\w)/)) {                  // NOTE: <<SOMETHING\n...\nSOMETHING\n
            stream.eatWhile(/\w/)
            return tokenSOMETHING(stream, state, stream.current().substr(2))
        }
        if (stream.sol() && stream.match(/^\=begin(?!\w)/)) {// NOTE: \n=item...\n=cut\n
            return tokenSOMETHING(stream, state, '=end')
        }
        var ch = stream.next()
        if (ch == '"' || ch == "'") {                           // NOTE: ' or " or <<'SOMETHING'\n...\nSOMETHING\n or <<"SOMETHING"\n...\nSOMETHING\n
            if (prefix(stream, 3) == "<<" + ch) {
                var p = stream.pos
                stream.eatWhile(/\w/)
                var n = stream.current().substr(1)
                if (n && stream.eat(ch))
                    return tokenSOMETHING(stream, state, n)
                stream.pos = p
            }
            return tokenChain(stream, state, [ch], "string")
        }
        if (ch == "q") {
            var c = look(stream, -2)
            if (!(c && /\w/.test(c))) {
                c = look(stream, 0)
                if (c == "x") {
                    c = look(stream, 1)
                    if (c == "(") {
                        eatSuffix(stream, 2)
                        return tokenChain(stream, state, [")"], RXstyle, RXmodifiers)
                    }
                    if (c == "[") {
                        eatSuffix(stream, 2)
                        return tokenChain(stream, state, ["]"], RXstyle, RXmodifiers)
                    }
                    if (c == "{") {
                        eatSuffix(stream, 2)
                        return tokenChain(stream, state, ["}"], RXstyle, RXmodifiers)
                    }
                    if (c == "<") {
                        eatSuffix(stream, 2)
                        return tokenChain(stream, state, [">"], RXstyle, RXmodifiers)
                    }
                    if (/[\^'"!~\/]/.test(c)) {
                        eatSuffix(stream, 1)
                        return tokenChain(stream, state, [stream.eat(c)], RXstyle, RXmodifiers)
                    }
                }
                else if (c == "q") {
                    c = look(stream, 1)
                    if (c == "(") {
                        eatSuffix(stream, 2)
                        return tokenChain(stream, state, [")"], "string")
                    }
                    if (c == "[") {
                        eatSuffix(stream, 2)
                        return tokenChain(stream, state, ["]"], "string")
                    }
                    if (c == "{") {
                        eatSuffix(stream, 2)
                        return tokenChain(stream, state, ["}"], "string")
                    }
                    if (c == "<") {
                        eatSuffix(stream, 2)
                        return tokenChain(stream, state, [">"], "string")
                    }
                    if (/[\^'"!~\/]/.test(c)) {
                        eatSuffix(stream, 1)
                        return tokenChain(stream, state, [stream.eat(c)], "string")
                    }
                }
                else if (c == "w") {
                    c = look(stream, 1)
                    if (c == "(") {
                        eatSuffix(stream, 2)
                        return tokenChain(stream, state, [")"], "bracket")
                    }
                    if (c == "[") {
                        eatSuffix(stream, 2)
                        return tokenChain(stream, state, ["]"], "bracket")
                    }
                    if (c == "{") {
                        eatSuffix(stream, 2)
                        return tokenChain(stream, state, ["}"], "bracket")
                    }
                    if (c == "<") {
                        eatSuffix(stream, 2)
                        return tokenChain(stream, state, [">"], "bracket")
                    }
                    if (/[\^'"!~\/]/.test(c)) {
                        eatSuffix(stream, 1)
                        return tokenChain(stream, state, [stream.eat(c)], "bracket")
                    }
                }
                else if (c == "r") {
                    c = look(stream, 1)
                    if (c == "(") {
                        eatSuffix(stream, 2)
                        return tokenChain(stream, state, [")"], RXstyle, RXmodifiers)
                    }
                    if (c == "[") {
                        eatSuffix(stream, 2)
                        return tokenChain(stream, state, ["]"], RXstyle, RXmodifiers)
                    }
                    if (c == "{") {
                        eatSuffix(stream, 2)
                        return tokenChain(stream, state, ["}"], RXstyle, RXmodifiers)
                    }
                    if (c == "<") {
                        eatSuffix(stream, 2)
                        return tokenChain(stream, state, [">"], RXstyle, RXmodifiers)
                    }
                    if (/[\^'"!~\/]/.test(c)) {
                        eatSuffix(stream, 1)
                        return tokenChain(stream, state, [stream.eat(c)], RXstyle, RXmodifiers)
                    }
                }
                else if (/[\^'"!~\/(\[{<]/.test(c)) {
                    if (c == "(") {
                        eatSuffix(stream, 1)
                        return tokenChain(stream, state, [")"], "string")
                    }
                    if (c == "[") {
                        eatSuffix(stream, 1)
                        return tokenChain(stream, state, ["]"], "string")
                    }
                    if (c == "{") {
                        eatSuffix(stream, 1)
                        return tokenChain(stream, state, ["}"], "string")
                    }
                    if (c == "<") {
                        eatSuffix(stream, 1)
                        return tokenChain(stream, state, [">"], "string")
                    }
                    if (/[\^'"!~\/]/.test(c)) {
                        return tokenChain(stream, state, [stream.eat(c)], "string")
                    }
                }
            }
        }
        if (ch == "m") {
            var c = look(stream, -2)
            if (!(c && /\w/.test(c))) {
                c = stream.eat(/[(\[{<\^'"!~\/]/)
                if (c) {
                    if (/[\^'"!~\/]/.test(c)) {
                        return tokenChain(stream, state, [c], RXstyle, RXmodifiers)
                    }
                    if (c == "(") {
                        return tokenChain(stream, state, [")"], RXstyle, RXmodifiers)
                    }
                    if (c == "[") {
                        return tokenChain(stream, state, ["]"], RXstyle, RXmodifiers)
                    }
                    if (c == "{") {
                        return tokenChain(stream, state, ["}"], RXstyle, RXmodifiers)
                    }
                    if (c == "<") {
                        return tokenChain(stream, state, [">"], RXstyle, RXmodifiers)
                    }
                }
            }
        }
        if (ch == "s") {
            var c = /[\/>\]})\w]/.test(look(stream, -2))
            if (!c) {
                c = stream.eat(/[(\[{<\^'"!~\/]/)
                if (c) {
                    if (c == "[")
                        return tokenChain(stream, state, ["]", "]"], RXstyle, RXmodifiers)
                    if (c == "{")
                        return tokenChain(stream, state, ["}", "}"], RXstyle, RXmodifiers)
                    if (c == "<")
                        return tokenChain(stream, state, [">", ">"], RXstyle, RXmodifiers)
                    if (c == "(")
                        return tokenChain(stream, state, [")", ")"], RXstyle, RXmodifiers)
                    return tokenChain(stream, state, [c, c], RXstyle, RXmodifiers)
                }
            }
        }
        if (ch == "y") {
            var c = /[\/>\]})\w]/.test(look(stream, -2))
            if (!c) {
                c = stream.eat(/[(\[{<\^'"!~\/]/)
                if (c) {
                    if (c == "[")
                        return tokenChain(stream, state, ["]", "]"], RXstyle, RXmodifiers)
                    if (c == "{")
                        return tokenChain(stream, state, ["}", "}"], RXstyle, RXmodifiers)
                    if (c == "<")
                        return tokenChain(stream, state, [">", ">"], RXstyle, RXmodifiers)
                    if (c == "(")
                        return tokenChain(stream, state, [")", ")"], RXstyle, RXmodifiers)
                    return tokenChain(stream, state, [c, c], RXstyle, RXmodifiers)
                }
            }
        }
        if (ch == "t") {
            var c = /[\/>\]})\w]/.test(look(stream, -2))
            if (!c) {
                c = stream.eat("r"); if (c) {
                    c = stream.eat(/[(\[{<\^'"!~\/]/)
                    if (c) {
                        if (c == "[")
                            return tokenChain(stream, state, ["]", "]"], RXstyle, RXmodifiers)
                        if (c == "{")
                            return tokenChain(stream, state, ["}", "}"], RXstyle, RXmodifiers)
                        if (c == "<")
                            return tokenChain(stream, state, [">", ">"], RXstyle, RXmodifiers)
                        if (c == "(")
                            return tokenChain(stream, state, [")", ")"], RXstyle, RXmodifiers)
                        return tokenChain(stream, state, [c, c], RXstyle, RXmodifiers)
                    }
                }
            }
        }
        if (ch == "`") {
            return tokenChain(stream, state, [ch], "variable-2")
        }
        if (ch == "/") {
            if (!/~\s*$/.test(prefix(stream)))
                return "operator"
            else
                return tokenChain(stream, state, [ch], RXstyle, RXmodifiers)
        }
        if (ch == "$") {
            var p = stream.pos
            if (stream.eatWhile(/\d/) || stream.eat("{") && stream.eatWhile(/\d/) && stream.eat("}"))
                return "variable-2"
            else
                stream.pos = p
        }
        if (/[$@%]/.test(ch)) {
            var p = stream.pos
            if (stream.eat(/[!.]/) && stream.eat(/[A-Za-z\-_']/) || !/[@$%&]/.test(look(stream, -2)) && stream.eat(/[=|\\\-#?@;:&`~\^!\[\]*'"$+.,\/<>()]/)) {
                var c = stream.current()
                if (PERL6[c])
                    return "variable-2"
            }
            stream.pos = p
        }
        if (/[$@%&]/.test(ch)) {
            if (stream.eatWhile(/[!.'\-\w$\[\]]/)) {
                var c = stream.current()
                if (PERL6[c])
                    return "variable-2"
                else
                    return "variable"
            }
        }
        if (ch == "#") {
            if (look(stream, -2) != "$") {
                stream.skipToEnd()
                return "comment"
            }
        }
        if (/[:+\-\^*$&%@=<>!?|\/~\.]/.test(ch)) {
            var p = stream.pos
            stream.eatWhile(/[:+\-\^*$&%@=<>!?|\/~\.]/)
            if (PERL6[stream.current()])
                return "operator"
            else
                stream.pos = p
        }
        if (ch == "_") {
            if (stream.pos == 1) {
                if (suffix(stream, 6) == "_END__") {
                    return tokenChain(stream, state, ['\0'], "comment")
                }
                else if (suffix(stream, 7) == "_DATA__") {
                    return tokenChain(stream, state, ['\0'], "variable-2")
                }
                else if (suffix(stream, 7) == "_C__") {
                    return tokenChain(stream, state, ['\0'], "string")
                }
            }
        }
        if (/\w/.test(ch)) {
            var p = stream.pos
            if (look(stream, -2) == "<" && (look(stream, 0) == ">" || stream.eatWhile(/\w/) && look(stream, 0) == ">"))
                return "string"
            else
                stream.pos = p
        }
        if (/[A-Z\-a-z]/.test(ch)) {
            var l = look(stream, -2)
            var p = stream.pos
            stream.eatWhile(/[A-Z_\-a-z]/)
            if (/[\da-z]/.test(look(stream, 0))) {
                stream.pos = p
            }
            else {
                var c = PERL6[stream.current()]
                if (!c)
                    return "meta"
                if (c[1])
                    c = c[0]
                if (l != ":") {
                    if (c == 1)
                        return "keyword"
                    else if (c == 2)
                        return "def"
                    else if (c == 3)
                        return "atom"
                    else if (c == 4)
                        return "operator"
                    else if (c == 5)
                        return "variable-2"
                    else
                        return "meta"
                }
                else
                    return "meta"
            }
        }
        if (/[a-zA-Z_\-']/.test(ch)) {
            var l = look(stream, -2)
            stream.eatWhile(/\w/)
            var c = PERL6[stream.current()]
            if (!c)
                return "meta"
            if (c[1])
                c = c[0]
            if (l != ":") {
                if (c == 1)
                    return "keyword"
                else if (c == 2)
                    return "def"
                else if (c == 3)
                    return "atom"
                else if (c == 4)
                    return "operator"
                else if (c == 5)
                    return "variable-2"
                else
                    return "meta"
            }
            else
                return "meta"
        }
        return null
    }

    return {
        startState: function () {
            return {
                tokenize: tokenPerl,
                chain: null,
                style: null,
                tail: null
            }
        },
        token: function (stream, state) {
            return (state.tokenize || tokenPerl)(stream, state)
        },
        lineComment: '#'
    }
})

CodeMirror.defineMIME("text/x-perl6", "perl6")

function look(stream, c) {
    return stream.string.charAt(stream.pos + (c || 0))
}

// return a part of prefix of current stream from current position
function prefix(stream, c) {
    if (c) {
        var x = stream.pos - c
        return stream.string.substr((x >= 0 ? x : 0), c);
    }
    else {
        return stream.string.substr(0, stream.pos - 1)
    }
}

// return a part of suffix of current stream from current position
function suffix(stream, c) {
    var y = stream.string.length
    var x = y - stream.pos + 1
    return stream.string.substr(stream.pos, (c && c < y ? c : x))
}

// eating and vomiting a part of stream from current position
function eatSuffix(stream, c) {
    var x = stream.pos + c
    var y
    if (x <= 0)
        stream.pos = 0
    else if (x >= (y = stream.string.length - 1))
        stream.pos = y
    else
        stream.pos = x
}
