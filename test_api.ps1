try {
    $response = Invoke-WebRequest -Uri "http://localhost:3000/" -Method POST -ContentType "application/json" -Body '{"GetMenu": {}}'
    Write-Host "Success! Response:"
    Write-Host $response.Content
} catch {
    Write-Host "Error: $($_.Exception.Message)"
}
