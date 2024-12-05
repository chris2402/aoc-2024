# start.ps1


# Get the directory of the executing script
$currentDirectory = $PSScriptRoot
Set-Location -Path $currentDirectory

# Load JSON from env.json file
$envJsonPath = Join-Path -Path $PSScriptRoot -ChildPath "env.json"
if (-not (Test-Path -Path $envJsonPath -PathType Leaf)) {
    Write-Error "env.json file not found."
    exit
}
$envData = Get-Content -Path $envJsonPath | ConvertFrom-Json


# Get the current date
$currentDate = Get-Date

# Loop through days 1 to 24
for ($day = 1; $day -le 24; $day++) {
    # Check if the current date is less than December $day, 2024
    $targetDate = Get-Date -Year 2024 -Month 12 -Day ($day)
    if ($targetDate -gt $currentDate) {
        Write-Output "Current date is less than December $day, 2024. Exiting..."
        exit
    }
    
    # Format the day with leading zero if necessary
    $folderName = "{0}_dec" -f $day
    $folderPath = Join-Path -Path $currentDirectory -ChildPath $folderName
    $cargoTomlPath = Join-Path -Path $folderPath -ChildPath "Cargo.toml"
    $inputPath = Join-Path -Path $folderPath -ChildPath "input.txt"
    $inputUrl = "https://adventofcode.com/2024/day/{0}/input" -f $day 

    # Check if the folder exists and if Cargo.toml contains the appended content
    if (-not (Test-Path -Path $folderPath -PathType Container ))
    {
        # Create the folder
        New-Item -ItemType Directory -Name $folderName
    }
    
    # Initiate the Cargo project
    if (-not (Test-Path -Path $cargoTomlPath -PathType Leaf))
    {
        # Change to the new directory
        Set-Location -Path $folderPath
        # Call cargo to initialize a new project
        cargo init --vcs none --name "aoc_$day"
        # Change back to the original directory
        Set-Location -Path $currentDirectory
    } 

    # Add load_input as a dependency
    if (-not (Select-String -Path $cargoTomlPath -Pattern "\[dependencies.load_input\]" -Quiet -ErrorAction SilentlyContinue)) {
        # Append to Cargo.toml
        Add-Content -Path $cargoTomlPath -Value "`n[dependencies.load_input]`npath = '../load_input'"
    }

    if (-not (Test-Path -Path $inputPath -PathType Leaf))
    {

        Invoke-WebRequest -Uri $inputUrl -OutFile $inputPath -Headers @{ "Cookie" = "session=$($envData.sessionToken)" }
    }

}

Write-Output "Folders for days 1 to 24 of December have been created."