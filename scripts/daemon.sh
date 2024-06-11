#!/bin/bash -e

SCRIPT_DIR=$(dirname "$0")
cd "$SCRIPT_DIR" && cd ..;

echo "Installing autorunner daemon..."
echo "Working directory: $(pwd)"

UNIT_FILENAME="autorunner.service"

UNIT_FILE_PATH="/etc/systemd/system/${UNIT_FILENAME}"

# Contenido del archivo de unidad
cat > "$UNIT_FILE_PATH" << EOF
[Unit]
Description=Github actions self-hosted runners
After=network.target

[Service]
ExecStart=$(pwd)/autorunner
WorkingDirectory=$(pwd)

[Install]
WantedBy=multi-user.target
EOF

echo "Unit file generated at: $UNIT_FILE_PATH"

cat $UNIT_FILE_PATH

# Reload systemd
systemctl daemon-reload
systemctl enable ${UNIT_FILENAME}
systemctl start ${UNIT_FILENAME}
systemctl restart ${UNIT_FILENAME}
systemctl status ${UNIT_FILENAME}

echo "Autorunner daemon installed successfully!"
