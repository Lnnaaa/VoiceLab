# VoiceLab

> ⚠️ **Ini adalah versi yang sudah dikustomisasi.**
> Source asli: [alexeygrigorev/microboost](https://github.com/alexeygrigorev/microboost) oleh [@alexeygrigorev](https://github.com/alexeygrigorev)

Aplikasi Windows untuk meningkatkan volume mikrofon secara real-time menggunakan pipeline audio melalui [VB-CABLE](https://vb-audio.com/Cable/).

## Perubahan dari versi asli

- Desain UI baru yang responsif (card-style layout, pill buttons, emoji icons)
- System tray berfungsi penuh: tekan X untuk sembunyikan ke tray, klik ikon tray untuk tampilkan kembali
- Toggle taskbar dinamis: window hilang dari taskbar saat di-minimize ke tray, muncul kembali saat dibuka dari tray
- Custom app icon

## Cara kerja

```
Microphone → [capture] → gain × boost → noise gate → ring buffer → [playback] → VB-CABLE
                                                                                    ↓
                                                                Discord/Teams/Zoom picks up
                                                                "CABLE Output" as microphone
```

1. Audio driver mengambil sampel mic via input callback
2. Setiap sampel dikalikan faktor boost (misal 2.0x) dan di-clamp ke [-1, 1]
3. Noise gate meredam suara di bawah noise floor yang sudah dikalibrasi
4. Sampel diproses masuk ke lock-free ring buffer
5. Output callback membaca dari ring buffer dan menulis ke VB-CABLE
6. Aplikasi seperti Discord melihat "CABLE Output" sebagai mikrofon

## Fitur

- Real-time microphone boost dari 0.1x hingga 5x (10% hingga 500%)
- Auto-kalibrasi level suara (~-16 dBFS)
- Noise gate adaptif
- Live waveform visualizer (input vs boosted output)
- Profil per-mikrofon (menyimpan boost & noise gate per perangkat)
- Deteksi hot-plug mikrofon
- Setup VB-CABLE otomatis saat pertama kali dijalankan
- Test recording & playback
- Lock-free audio pipeline (96.7 dB SNR)
- Native UI dengan egui

## Instalasi

Build dari source:

```bash
git clone https://github.com/Lnnaaa/VoiceLab
cd voicelab
cargo build --release
```

Atau download langsung dari [Release](https://github.com/Lnnaaa/VoiceLab/releases/download/v0.1.1/VoiceLab.exe)

> **Catatan:** Butuh Visual C++ Redistributable. Download di: https://aka.ms/vs/17/release/vc_redist.x64.exe

## Cara penggunaan

1. Jalankan VoiceLab. Jika VB-CABLE belum terinstall, klik "Install VB-CABLE".
2. Pilih mikrofon dari dropdown.
3. Klik "Auto-Calibrate" atau set boost manual.
4. Klik "Start Boost".
5. Di Discord/Teams/Zoom, pilih "CABLE Output" sebagai input mikrofon.

Rekaman tersimpan di `%APPDATA%\Microboost\`.

## Kebutuhan sistem

- Windows 10 atau lebih baru
- VB-CABLE (otomatis terinstall saat pertama kali launch)

## Tech Stack

- [egui](https://github.com/emilk/egui) — Native GUI
- [cpal](https://github.com/RustAudio/cpal) — Audio capture & playback
- [hound](https://github.com/ruuda/hound) — WAV encoding/decoding
- [tray-icon](https://github.com/tauri-apps/tray-icon) — System tray
- [VB-CABLE](https://vb-audio.com/Cable/) — Virtual audio cable

## Lisensi

MIT — sama seperti source asli.

---

*Dikustomisasi oleh Lana dari proyek asli [alexeygrigorev/microboost](https://github.com/alexeygrigorev/microboost).*
