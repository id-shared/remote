# Total duration in seconds (7 hours = 25200 seconds)
$totalDuration = 2 * 10 * 86400

# Number of progress steps (100 = 1% increments)
$totalSteps = 100

# Delay between each step
$delay = $totalDuration / $totalSteps

for ($i = 93; $i -le $totalSteps; $i++) {
    $percent = [int](($i / $totalSteps) * 100)
    Write-Progress -Activity "Building" -Status "$percent% Complete" -PercentComplete $percent
    Start-Sleep -Seconds $delay
}

Write-Progress -Activity "Building" -Completed -Status "Done"
Write-Host "Completed!"
