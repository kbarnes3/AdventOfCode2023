$ProjectRoot = $PSScriptRoot

$Programs =
    @{"name" = "day01_trebuchet_part1"; "output" = "54632"},
    @{"name" = "day01_trebuchet_part2"; "output" = "54019"},
    @{"name" = "day02_cube_conundrum_part1"; "output" = "2879"},
    @{"name" = "day02_cube_conundrum_part2"; "output" = "65122"},
    @{"name" = "day03_gear_ratios_part1"; "output" = "540131"},
    @{"name" = "day03_gear_ratios_part2"; "output" = "86879020"},
    @{"name" = "day04_scratchcards_part1"; "output" = "20117"},
    @{"name" = "day04_scratchcards_part2"; "output" = "13768818"},
    @{"name" = "day05_if_you_give_a_seed_a_fertilizer_part1"; "output" = "346433842"},
    @{"name" = "day05_if_you_give_a_seed_a_fertilizer_part2"; "output" = "60294664"},
    @{"name" = "day06_wait_for_it_part1"; "output" = "227850"},
    @{"name" = "day06_wait_for_it_part2"; "output" = "42948149"},
    @{"name" = "day07_camel_cards_part1"; "output" = "253954294"},
    @{"name" = "day07_camel_cards_part2"; "output" = "254837398"},
    @{"name" = "day08_haunted_wasteland_part1"; "output" = "18023"},
    @{"name" = "day08_haunted_wasteland_part2"; "output" = "14449445933179"},
    @{"name" = "day09_mirage_maintenance_part1"; "output" = "1987402313"}

for ($i = 0; $i -lt $Programs.Length; $i++) {
    $program = $Programs[$i]
    $name = $program["name"]
    $expectedOutput = $program["output"]

    $currentOperation = "Testing $name"
    $percent = $i / $Programs.Length * 100
    $status = "$($i + 1)/$($Programs.Length) $name"
    Write-Progress -Id 0 -Activity "Test Solutions" -CurrentOperation $currentOperation -PercentComplete $percent -Status $status

    Push-Location (Join-Path $ProjectRoot $name)
    Write-Host "`u{1F9F1} Building $name"
    & cargo fmt --all -- --check
    if (-not $?) {
        throw "Error running cargo fmt for $name"
    }
    & cargo clippy --all-targets --all-features -- -D warnings
    if (-not $?) {
        throw "Error running cargo clippy for $name"
    }
    & cargo check
    if (-not $?) {
        throw "Error running cargo check for $name"
    }
    & cargo build --release
    if (-not $?) {
        throw "Error building $name"
    }
    $binary_name = "$($name).exe"
    $binary_path = Join-Path ".\target\release" $binary_name
    Write-Host "`u{1F3C3} Running $name"
    $output = & $binary_path
    if ($output -ne $expectedOutput) {
        throw "Expected $expectedOutput, got $output"
    }
    Write-Host "✔️ $name passed!" -ForegroundColor Green
    Pop-Location
}

Write-Progress -Id 0 -Activity " " -PercentComplete 100
Write-Progress -Id 0 -Activity " " -Completed
