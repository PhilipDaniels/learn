"12 is a $((12).GetType().Name)"        # Int32 literal (Int64 if necessary).
"12.0 is a $((12.0).GetType().Name)"    # System.Double
"12.0d is a $((12.0d).GetType().Name)"  # System.Decimal

"`nNumeric suffixes use powers-of-two:

2kb = $(2kb)              2KB = $(2KB)                2.5kb = $(2.5kb)
2mb = $(2mb)           2MB = $(2MB)             2.5mb = $(2.5mb)
2gb = $(2gb)        2GB = $(2GB)          2.5gb = $(2.5gb)
2tb = $(2tb)     2TB = $(2TB)       2.5tb = $(2.5tb)"
