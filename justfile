[parallel]
main: ui api
# Run ui
ui:
    unbuffer cargo tauri dev | while read line; do echo "[UI] $line"; done

# Run api
api:
    unbuffer cargo run --package api | while read line; do echo "[API] $line"; done


