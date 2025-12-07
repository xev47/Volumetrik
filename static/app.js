const API_URL = '/api';

let typesBarChart = null;
let typesDoughnutChart = null;
let currentData = null;
let abortController = null;
let currentSort = { column: 'size', direction: 'desc' };
let grid = null;

const translations = {
    en: {
        scan: "Analyze",
        largest_directories: "Top Directories by Size",
        name: "Filename",
        size: "Storage Usage",
        items: "Item Count",
        top_file_types: "File Type Distribution",
        file_type_usage: "Storage Allocation by Type",
        available_space: "Free Storage Capacity",
        used: "Allocated",
        available: "Free",
        total: "Capacity",
        largest_files: "Top Files by Size",
        last_accessed: "Last Access Timestamp",
        up: "Parent Directory",
        percent: "Usage %",
        files: "File Count",
        modified: "Last Modified",
        select_folder: "Select Target Directory",
        select_this_folder: "Set Target",
        settings: "Configuration",
        general: "System",
        monitoring: "Watchdog",
        alerts: "Notifications",
        about: "Info",
        general_settings: "System Configuration",
        general_settings_desc: "Adjust core application parameters.",
        language: "Interface Language",
        folder_monitoring: "Directory Watchdog",
        folder_monitoring_desc: "Setup automated storage monitoring thresholds.",
        enable_monitoring: "Activate Watchdog",
        enable_monitoring_desc: "Enable background storage analysis",
        monitored_paths: "Watch List",
        max_used: "Max Allocation",
        min_remaining: "Min Free Space",
        add: "Append",
        interval_minutes: "Polling Interval (min)",
        notifications: "Alerting System",
        notifications_desc: "Configure triggers for threshold violations.",
        enable_alerts: "Activate Alerting",
        enable_alerts_desc: "Dispatch events on trigger",
        custom_alert_message: "Custom Payload Template",
        available_variables: "Variables: {path}, {threshold}",
        bot_token: "API Token",
        chat_id: "Channel ID",
        user_key: "User Key",
        api_token: "API Token",
        server_url: "Endpoint URL",
        app_token: "Application Token",
        topic_url: "Topic Endpoint",
        access_token: "Bearer Token (Opt)",
        generic_webhook_url: "Webhook Endpoint",
        slack_webhook_url: "Slack Webhook Endpoint",
        discord_webhook_url: "Discord Webhook Endpoint",
        teams_webhook_url: "Teams Webhook Endpoint",
        app_description: "Storage Analysis & Monitoring System",
        cancel: "Abort",
        save_changes: "Commit Changes",
        settings_saved: "Configuration Persisted",
        enter_path: "Input directory path...",
        path_placeholder: "/mnt/data",
        alert_message_placeholder: "Alert: Storage quota exceeded for {path} (> {threshold}GB)",
        scan_started: "Scan started for: ",
        scan_failed: "Scan failed: ",
        scan_aborted: "Scan aborted",
        error_selecting_folder: "Error selecting folder: ",
        path_already_monitored: "Path already monitored",
        invalid_path_threshold: "Please enter a valid path and threshold value",
        settings_saved_success: "Settings saved successfully!",
        settings_save_failed: "Failed to save settings: ",
        layout_saved: "Layout saved",
        error_saving_layout: "Error saving layout: ",
        error_loading_layout: "Error loading layout: ",
        color_palette: "Color Palette",
        palette_default: "Default (Slate)",
        palette_ocean: "Ocean (Blue/Cyan)",
        palette_sunset: "Sunset (Orange/Red)",
        palette_forest: "Forest (Green)",
        palette_purple: "Royal (Purple)",
        file_browser: "File Browser",
        reset_layout: "Reset Layout",
        layout_reset: "Layout reset to default"
    },
    fr: {
        scan: "Analyser",
        largest_directories: "Répertoires Volumineux",
        name: "Nom de Fichier",
        size: "Occupation",
        items: "Nombre d'éléments",
        top_file_types: "Distribution par Type",
        file_type_usage: "Allocation par Extension",
        available_space: "Capacité Libre",
        used: "Alloué",
        available: "Libre",
        total: "Capacité Totale",
        largest_files: "Fichiers Volumineux",
        last_accessed: "Dernier Accès",
        up: "Répertoire Parent",
        percent: "% Usage",
        files: "Nb Fichiers",
        modified: "Dernière Modif.",
        select_folder: "Sélectionner Répertoire Cible",
        select_this_folder: "Définir Cible",
        settings: "Configuration",
        general: "Système",
        monitoring: "Supervision",
        alerts: "Notifications",
        about: "Info Système",
        general_settings: "Configuration Système",
        general_settings_desc: "Ajuster les paramètres globaux.",
        language: "Langue Interface",
        folder_monitoring: "Supervision de Répertoires",
        folder_monitoring_desc: "Configurer les seuils de supervision du stockage.",
        enable_monitoring: "Activer Supervision",
        enable_monitoring_desc: "Activer l'analyse en arrière-plan",
        monitored_paths: "Liste de Supervision",
        max_used: "Allocation Max",
        min_remaining: "Espace Libre Min",
        add: "Ajouter Entrée",
        interval_minutes: "Intervalle de Polling (min)",
        notifications: "Système d'Alerte",
        notifications_desc: "Configurer les déclencheurs d'alerte.",
        enable_alerts: "Activer Alertes",
        enable_alerts_desc: "Expédier les événements sur déclenchement",
        custom_alert_message: "Modèle de Charge Utile",
        available_variables: "Variables : {path}, {threshold}",
        bot_token: "Token API",
        chat_id: "ID Canal",
        user_key: "Clé Utilisateur",
        api_token: "Token API",
        server_url: "URL Endpoint",
        app_token: "Token Application",
        topic_url: "Endpoint Topic",
        access_token: "Token Bearer (Opt)",
        generic_webhook_url: "Endpoint Webhook",
        slack_webhook_url: "Endpoint Webhook Slack",
        discord_webhook_url: "Endpoint Webhook Discord",
        teams_webhook_url: "Endpoint Webhook Teams",
        app_description: "Système d'Analyse et Supervision de Stockage",
        cancel: "Abandonner",
        save_changes: "Appliquer Changements",
        settings_saved: "Configuration Persistée",
        enter_path: "Saisir chemin répertoire...",
        path_placeholder: "/mnt/data",
        alert_message_placeholder: "Alerte : Quota de stockage dépassé pour {path} (> {threshold}Go)",
        scan_started: "Analyse démarrée pour : ",
        scan_failed: "Échec de l'analyse : ",
        scan_aborted: "Analyse annulée",
        error_selecting_folder: "Erreur lors de la sélection du dossier : ",
        path_already_monitored: "Chemin déjà surveillé",
        invalid_path_threshold: "Veuillez entrer un chemin et un seuil valides",
        settings_saved_success: "Paramètres enregistrés avec succès !",
        settings_save_failed: "Échec de l'enregistrement des paramètres : ",
        layout_saved: "Disposition enregistrée",
        error_saving_layout: "Erreur lors de l'enregistrement de la disposition : ",
        error_loading_layout: "Erreur lors du chargement de la disposition : ",
        color_palette: "Palette de Couleurs",
        palette_default: "Défaut (Ardoise)",
        palette_ocean: "Océan (Bleu/Cyan)",
        palette_sunset: "Coucher de Soleil (Orange/Rouge)",
        palette_forest: "Forêt (Vert)",
        palette_purple: "Royal (Violet)",
        file_browser: "Explorateur de Fichiers",
        reset_layout: "Réinitialiser la disposition",
        layout_reset: "Disposition réinitialisée"
    },
    es: {
        scan: "Analizar",
        largest_directories: "Directorios Voluminosos",
        name: "Nombre de Archivo",
        size: "Ocupación",
        items: "Conteo de Ítems",
        top_file_types: "Distribución por Tipo",
        file_type_usage: "Asignación por Extensión",
        available_space: "Capacidad Libre",
        used: "Asignado",
        available: "Libre",
        total: "Capacidad Total",
        largest_files: "Archivos Voluminosos",
        last_accessed: "Último Acceso",
        up: "Directorio Padre",
        percent: "% Uso",
        files: "Conteo Archivos",
        modified: "Última Modif.",
        select_folder: "Seleccionar Directorio Objetivo",
        select_this_folder: "Establecer Objetivo",
        settings: "Configuración",
        general: "Sistema",
        monitoring: "Supervisión",
        alerts: "Notificaciones",
        about: "Info Sistema",
        general_settings: "Configuración del Sistema",
        general_settings_desc: "Ajustar parámetros globales.",
        language: "Idioma Interfaz",
        folder_monitoring: "Supervisión de Directorios",
        folder_monitoring_desc: "Configurar umbrales de supervisión de almacenamiento.",
        enable_monitoring: "Activar Supervisión",
        enable_monitoring_desc: "Habilitar análisis en segundo plano",
        monitored_paths: "Lista de Supervisión",
        max_used: "Asignación Máx",
        min_remaining: "Espacio Libre Mín",
        add: "Añadir Entrada",
        interval_minutes: "Intervalo de Polling (min)",
        notifications: "Sistema de Alertas",
        notifications_desc: "Configurar disparadores de alerta.",
        enable_alerts: "Activar Alertas",
        enable_alerts_desc: "Despachar eventos al disparar",
        custom_alert_message: "Plantilla de Carga Útil",
        available_variables: "Variables: {path}, {threshold}",
        bot_token: "Token API",
        chat_id: "ID Canal",
        user_key: "Clave Usuario",
        api_token: "Token API",
        server_url: "URL Endpoint",
        app_token: "Token Aplicación",
        topic_url: "Endpoint Topic",
        access_token: "Token Bearer (Opt)",
        generic_webhook_url: "Endpoint Webhook",
        slack_webhook_url: "Endpoint Webhook Slack",
        discord_webhook_url: "Endpoint Webhook Discord",
        teams_webhook_url: "Endpoint Webhook Teams",
        app_description: "Sistema de Análisis y Supervisión de Almacenamiento",
        cancel: "Abortar",
        save_changes: "Aplicar Cambios",
        settings_saved: "Configuración Persistida",
        enter_path: "Ingresar ruta directorio...",
        path_placeholder: "/mnt/data",
        alert_message_placeholder: "Alerta: Cuota de almacenamiento excedida para {path} (> {threshold}GB)",
        scan_started: "Escaneo iniciado para: ",
        scan_failed: "Escaneo fallido: ",
        scan_aborted: "Escaneo abortado",
        error_selecting_folder: "Error al seleccionar carpeta: ",
        path_already_monitored: "Ruta ya monitoreada",
        invalid_path_threshold: "Por favor ingrese una ruta y un valor de umbral válidos",
        settings_saved_success: "¡Configuración guardada exitosamente!",
        settings_save_failed: "Error al guardar la configuración: ",
        layout_saved: "Diseño guardado",
        error_saving_layout: "Error al guardar el diseño: ",
        error_loading_layout: "Error al cargar el diseño: ",
        color_palette: "Paleta de Colores",
        palette_default: "Por Defecto (Pizarra)",
        palette_ocean: "Océano (Azul/Cian)",
        palette_sunset: "Atardecer (Naranja/Rojo)",
        palette_forest: "Bosque (Verde)",
        palette_purple: "Real (Púrpura)",
        file_browser: "Explorador de Archivos"
    },
    de: {
        scan: "Analysieren",
        largest_directories: "Voluminöse Verzeichnisse",
        name: "Dateiname",
        size: "Belegung",
        items: "Elementanzahl",
        top_file_types: "Verteilung nach Typ",
        file_type_usage: "Zuweisung nach Erweiterung",
        available_space: "Freie Kapazität",
        used: "Zugewiesen",
        available: "Frei",
        total: "Gesamtkapazität",
        largest_files: "Voluminöse Dateien",
        last_accessed: "Letzter Zugriff",
        up: "Übergeordnetes Verzeichnis",
        percent: "% Nutzung",
        files: "Dateianzahl",
        modified: "Letzte Änd.",
        select_folder: "Zielverzeichnis Wählen",
        select_this_folder: "Ziel Setzen",
        settings: "Konfiguration",
        general: "System",
        monitoring: "Überwachung",
        alerts: "Benachrichtigungen",
        about: "Systeminfo",
        general_settings: "Systemkonfiguration",
        general_settings_desc: "Globale Parameter anpassen.",
        language: "Schnittstellensprache",
        folder_monitoring: "Verzeichnisüberwachung",
        folder_monitoring_desc: "Schwellenwerte für Speicherüberwachung konfigurieren.",
        enable_monitoring: "Überwachung Aktivieren",
        enable_monitoring_desc: "Hintergrundanalyse aktivieren",
        monitored_paths: "Überwachungsliste",
        max_used: "Max Zuweisung",
        min_remaining: "Min Freier Speicher",
        add: "Eintrag Hinzufügen",
        interval_minutes: "Polling-Intervall (Min)",
        notifications: "Warnsystem",
        notifications_desc: "Auslöser für Warnungen konfigurieren.",
        enable_alerts: "Warnungen Aktivieren",
        enable_alerts_desc: "Ereignisse bei Auslösung versenden",
        custom_alert_message: "Nutzlast-Vorlage",
        available_variables: "Variablen: {path}, {threshold}",
        bot_token: "API Token",
        chat_id: "Kanal ID",
        user_key: "Benutzerschlüssel",
        api_token: "API Token",
        server_url: "Endpoint URL",
        app_token: "Anwendungs-Token",
        topic_url: "Topic Endpoint",
        access_token: "Bearer Token (Opt)",
        generic_webhook_url: "Webhook Endpoint",
        slack_webhook_url: "Slack Webhook Endpoint",
        discord_webhook_url: "Discord Webhook Endpoint",
        teams_webhook_url: "Teams Webhook Endpoint",
        app_description: "Speicheranalyse- & Überwachungssystem",
        cancel: "Abbrechen",
        save_changes: "Änderungen Anwenden",
        settings_saved: "Konfiguration Gespeichert",
        enter_path: "Verzeichnispfad eingeben...",
        path_placeholder: "/mnt/data",
        alert_message_placeholder: "Warnung: Speicherquote für {path} überschritten (> {threshold}GB)",
        scan_started: "Scan gestartet für: ",
        scan_failed: "Scan fehlgeschlagen: ",
        scan_aborted: "Scan abgebrochen",
        error_selecting_folder: "Fehler bei der Ordnerauswahl: ",
        path_already_monitored: "Pfad wird bereits überwacht",
        invalid_path_threshold: "Bitte geben Sie einen gültigen Pfad und Schwellenwert ein",
        settings_saved_success: "Einstellungen erfolgreich gespeichert!",
        settings_save_failed: "Fehler beim Speichern der Einstellungen: ",
        layout_saved: "Layout gespeichert",
        error_saving_layout: "Fehler beim Speichern des Layouts: ",
        error_loading_layout: "Fehler beim Laden des Layouts: ",
        color_palette: "Farbpalette",
        palette_default: "Standard (Schiefer)",
        palette_ocean: "Ozean (Blau/Cyan)",
        palette_sunset: "Sonnenuntergang (Orange/Rot)",
        palette_forest: "Wald (Grün)",
        palette_purple: "Königlich (Lila)",
        file_browser: "Dateibrowser"
    },
    it: {
        scan: "Analizza",
        largest_directories: "Directory più grandi",
        name: "Nome file",
        size: "Utilizzo spazio",
        items: "Numero elementi",
        top_file_types: "Distribuzione tipi file",
        file_type_usage: "Allocazione per tipo",
        available_space: "Spazio disponibile",
        used: "Usato",
        available: "Libero",
        total: "Capacità totale",
        largest_files: "File più grandi",
        last_accessed: "Ultimo accesso",
        up: "Directory superiore",
        percent: "% Uso",
        files: "Num file",
        modified: "Ultima modifica",
        select_folder: "Seleziona directory",
        select_this_folder: "Imposta destinazione",
        settings: "Configurazione",
        general: "Sistema",
        monitoring: "Monitoraggio",
        alerts: "Notifiche",
        about: "Info",
        general_settings: "Configurazione sistema",
        general_settings_desc: "Regola parametri principali.",
        language: "Lingua interfaccia",
        folder_monitoring: "Monitoraggio directory",
        folder_monitoring_desc: "Imposta soglie monitoraggio archiviazione.",
        enable_monitoring: "Attiva monitoraggio",
        enable_monitoring_desc: "Abilita analisi in background",
        monitored_paths: "Lista monitoraggio",
        max_used: "Allocazione Max",
        min_remaining: "Spazio libero Min",
        add: "Aggiungi",
        interval_minutes: "Intervallo polling (min)",
        notifications: "Sistema notifiche",
        notifications_desc: "Configura trigger per violazione soglie.",
        enable_alerts: "Attiva avvisi",
        enable_alerts_desc: "Invia eventi su trigger",
        custom_alert_message: "Modello payload personalizzato",
        available_variables: "Variabili: {path}, {threshold}",
        bot_token: "Token API",
        chat_id: "ID Canale",
        user_key: "Chiave Utente",
        api_token: "Token API",
        server_url: "URL Endpoint",
        app_token: "Token Applicazione",
        topic_url: "Endpoint Topic",
        access_token: "Token Bearer (Opz)",
        generic_webhook_url: "Endpoint Webhook",
        slack_webhook_url: "Endpoint Webhook Slack",
        discord_webhook_url: "Endpoint Webhook Discord",
        teams_webhook_url: "Endpoint Webhook Teams",
        app_description: "Sistema di analisi e monitoraggio archiviazione",
        cancel: "Annulla",
        save_changes: "Salva modifiche",
        settings_saved: "Configurazione salvata",
        enter_path: "Inserisci percorso directory...",
        path_placeholder: "/mnt/data",
        alert_message_placeholder: "Avviso: Quota archiviazione superata per {path} (> {threshold}GB)",
        scan_started: "Scansione avviata per: ",
        scan_failed: "Scansione fallita: ",
        scan_aborted: "Scansione annullata",
        error_selecting_folder: "Errore selezione cartella: ",
        path_already_monitored: "Percorso già monitorato",
        invalid_path_threshold: "Inserisci un percorso e una soglia validi",
        settings_saved_success: "Impostazioni salvate con successo!",
        settings_save_failed: "Salvataggio impostazioni fallito: ",
        layout_saved: "Layout salvato",
        error_saving_layout: "Errore salvataggio layout: ",
        error_loading_layout: "Errore caricamento layout: ",
        color_palette: "Tavolozza colori",
        palette_default: "Predefinito (Ardesia)",
        palette_ocean: "Oceano (Blu/Ciano)",
        palette_sunset: "Tramonto (Arancione/Rosso)",
        palette_forest: "Foresta (Verde)",
        palette_purple: "Reale (Viola)",
        file_browser: "Esplora File"
    }
};

function updateLanguage(lang) {
    if (!translations[lang]) return;
    
    document.querySelectorAll('[data-i18n]').forEach(el => {
        const key = el.getAttribute('data-i18n');
        if (translations[lang][key]) {
            el.textContent = translations[lang][key];
        }
    });

    document.querySelectorAll('[data-i18n-placeholder]').forEach(el => {
        const key = el.getAttribute('data-i18n-placeholder');
        if (translations[lang][key]) {
            el.placeholder = translations[lang][key];
        }
    });
}

function getTranslation(key) {
    const lang = initialLanguage || 'en';
    return (translations[lang] && translations[lang][key]) || translations['en'][key] || key;
}

document.addEventListener('DOMContentLoaded', () => {
    // Initialize Gridstack
    grid = GridStack.init({
        cellHeight: 50, // Finer granularity for resizing
        margin: 20,     // Larger gap between widgets
        animate: true,
        column: 12,
        disableOneColumnMode: false,
        float: true,    // Allow widgets to stay where placed (no auto-up)
        resizable: {
            handles: 'se'
        },
        alwaysShowResizeHandle: true
    });

    loadLayout();

    let saveTimeout;
    grid.on('change', (event, items) => {
        clearTimeout(saveTimeout);
        saveTimeout = setTimeout(saveLayout, 1000);
    });

    grid.on('resizestop', (event, el) => {
        if (el.querySelector('canvas')) {
            if (typesBarChart) typesBarChart.resize();
            if (typesDoughnutChart) typesDoughnutChart.resize();
        }
    });

    const scanBtn = document.getElementById('scanBtn');
    const browseBtn = document.getElementById('browseBtn');
    const pathInput = document.getElementById('pathInput');
    const themeBtn = document.getElementById('themeBtn');

    // Auto-scan root if empty
    if (!pathInput.value) {
        // scan("/");
    }

    scanBtn.addEventListener('click', () => {
        if (abortController) {
            abortController.abort();
            abortController = null;
            setLoading(false);
        } else {
            scan(pathInput.value);
        }
    });
    
    browseBtn.addEventListener('click', async () => {
        try {
            const response = await fetch(`${API_URL}/select-folder`);
            if (response.ok) {
                const data = await response.json();
                if (data.supported === false) {
                    openBrowseModal();
                } else if (data.path) {
                    pathInput.value = data.path;
                    scan(data.path);
                }
            } else {
                openBrowseModal();
            }
        } catch (error) {
            console.error('Error selecting folder:', error);
            showToast(getTranslation('error_selecting_folder') + error.message, 'error');
            openBrowseModal();
        }
    });

    pathInput.addEventListener('keypress', (e) => {
        if (e.key === 'Enter') scan(pathInput.value);
    });

    themeBtn.addEventListener('click', toggleTheme);

    // Initial theme check
    const savedTheme = localStorage.getItem('theme');
    if (savedTheme === 'dark' || !savedTheme) {
        document.body.setAttribute('data-theme', 'dark');
    }

    // Sorting listeners
    document.querySelectorAll('th[data-sort]').forEach(th => {
        th.addEventListener('click', () => {
            const column = th.dataset.sort;
            if (currentSort.column === column) {
                currentSort.direction = currentSort.direction === 'asc' ? 'desc' : 'asc';
            } else {
                currentSort.column = column;
                if (column === 'name') currentSort.direction = 'asc';
                else currentSort.direction = 'desc';
            }
            updateSortIcons();
            if (currentData) {
                sortFiles();
                renderFileBrowser(currentData.files, currentData.total_size);
            }
        });
    });
    
    // Load initial settings to apply language
    loadSettings();
    
    // Load palette
    const savedPalette = localStorage.getItem('palette') || 'default';
    applyPalette(savedPalette);
});

function applyPalette(palette) {
    document.body.setAttribute('data-palette', palette);
    localStorage.setItem('palette', palette);
    
    // Update charts if they exist to reflect new colors
    if (currentData) {
        const fileStats = processFileTypes(currentData.files);
        renderTopFileTypes(fileStats);
        renderFileTypeUsage(fileStats, currentData.total_size);
    }
}

function getChartColors() {
    const style = getComputedStyle(document.body);
    return [
        style.getPropertyValue('--chart-1').trim(),
        style.getPropertyValue('--chart-2').trim(),
        style.getPropertyValue('--chart-3').trim(),
        style.getPropertyValue('--chart-4').trim(),
        style.getPropertyValue('--chart-5').trim(),
        style.getPropertyValue('--chart-6').trim()
    ];
}

async function scan(path) {
    if (!path) return;

    if (abortController) {
        abortController.abort();
    }
    abortController = new AbortController();

    setLoading(true);
    showToast(getTranslation('scan_started') + path, 'info');

    try {
        const response = await fetch(`${API_URL}/scan?path=${encodeURIComponent(path)}`, {
            signal: abortController.signal
        });
        if (!response.ok) throw new Error('Scan failed');
        
        const data = await response.json();
        currentData = data;
        
        sortFiles();
        renderDashboard(data);
    } catch (error) {
        if (error.name === 'AbortError') {
            console.log('Scan aborted');
            showToast(getTranslation('scan_aborted'), 'info');
        } else {
            console.error(error);
            showToast(getTranslation('scan_failed') + error.message, 'error');
        }
    } finally {
        if (abortController && !abortController.signal.aborted) {
            abortController = null;
            setLoading(false);
        }
    }
}

function sortFiles() {
    if (!currentData || !currentData.files) return;
    
    const { column, direction } = currentSort;
    const m = direction === 'asc' ? 1 : -1;

    currentData.files.sort((a, b) => {
        let valA, valB;
        switch(column) {
            case 'name': valA = a.name.toLowerCase(); valB = b.name.toLowerCase(); break;
            case 'size': valA = a.size; valB = b.size; break;
            case 'percent': valA = a.size; valB = b.size; break;
            case 'files': valA = a.file_count; valB = b.file_count; break;
            case 'modified': valA = a.modified; valB = b.modified; break;
            default: return 0;
        }
        if (valA < valB) return -1 * m;
        if (valA > valB) return 1 * m;
        return 0;
    });
}

function updateSortIcons() {
    document.querySelectorAll('th[data-sort] i').forEach(icon => {
        icon.className = 'fas fa-sort';
    });
    const activeTh = document.querySelector(`th[data-sort="${currentSort.column}"]`);
    if (activeTh) {
        const icon = activeTh.querySelector('i');
        icon.className = currentSort.direction === 'asc' ? 'fas fa-sort-up' : 'fas fa-sort-down';
    }
}

// --- New Dashboard Rendering Logic ---

function renderDashboard(data) {
    // Update Header Info
    document.getElementById('pathInput').value = data.current;
    document.getElementById('currentPath').textContent = data.current;

    // Process Data
    const fileStats = processFileTypes(data.files);
    
    // Render Components
    renderFileTypeUsage(fileStats, data.total_size);
    renderDiskSpace(data); 
    renderTopFileTypes(fileStats);
    renderLargestFiles(data.files);
    renderLargestDirs(data.files);
    renderFileBrowser(data.files, data.total_size);
}

function processFileTypes(files) {
    const stats = {};
    files.forEach(f => {
        if (f.is_dir) return;
        const ext = f.name.includes('.') ? f.name.split('.').pop().toLowerCase() : 'other';
        if (!stats[ext]) stats[ext] = { size: 0, count: 0, ext: ext };
        stats[ext].size += f.size;
        stats[ext].count++;
    });
    
    // Convert to array and sort by size
    return Object.values(stats).sort((a, b) => b.size - a.size);
}

function renderFileTypeUsage(stats, totalSize) {
    const barContainer = document.getElementById('fileTypeBar');
    const legendContainer = document.getElementById('fileTypeLegend');
    barContainer.innerHTML = '';
    legendContainer.innerHTML = '';

    if (!stats || stats.length === 0 || totalSize === 0) {
        barContainer.innerHTML = '<div style="width: 100%; text-align: center; color: #7f8c8d; font-size: 0.9rem; padding: 2px;">No file type data available</div>';
        return;
    }

    const colors = getChartColors();
    
    // Take top 5 + other
    let topStats = stats.slice(0, 5);
    const otherSize = stats.slice(5).reduce((acc, s) => acc + s.size, 0);
    if (otherSize > 0) topStats.push({ ext: 'other', size: otherSize, count: 0 });

    topStats.forEach((stat, index) => {
        const percent = totalSize > 0 ? (stat.size / totalSize) * 100 : 0;
        if (percent < 0.5) return; // Skip tiny segments

        const color = colors[index % colors.length];
        
        // Bar Segment
        const segment = document.createElement('div');
        segment.className = 'progress-segment';
        segment.style.width = `${percent}%`;
        segment.style.backgroundColor = color;
        segment.title = `${stat.ext}: ${formatBytes(stat.size)} (${percent.toFixed(1)}%)`;
        barContainer.appendChild(segment);

        // Legend Item
        const item = document.createElement('div');
        item.className = 'legend-item';
        item.innerHTML = `
            <div class="legend-color" style="background-color: ${color}"></div>
            <span>${stat.ext}: ${formatBytes(stat.size)} (${percent.toFixed(1)}%)</span>
        `;
        legendContainer.appendChild(item);
    });
}

function renderDiskSpace(data) {
    const bar = document.getElementById('diskSpaceBar');
    const usedText = document.getElementById('diskUsed');
    const availText = document.getElementById('diskAvailable');
    const totalText = document.getElementById('diskTotal');

    if (data.disk_total && data.disk_available) {
        const total = data.disk_total;
        const available = data.disk_available;
        const used = total - available;
        const percent = (used / total) * 100;

        bar.style.width = `${percent}%`;
        usedText.textContent = `Used: ${formatBytes(used)}`;
        availText.textContent = `Available: ${formatBytes(available)}`;
        totalText.textContent = `Total: ${formatBytes(total)}`;
    } else {
        // Fallback if disk info is not available
        bar.style.width = '0%';
        usedText.textContent = 'Used: -';
        availText.textContent = 'Available: -';
        totalText.textContent = 'Disk capacity unknown';
    }
}

function renderTopFileTypes(stats) {
    const canvasBar = document.getElementById('typesBarChart');
    const canvasDoughnut = document.getElementById('typesDoughnutChart');

    if (!canvasBar || !canvasDoughnut) return;

    const ctxBar = canvasBar.getContext('2d');
    const ctxDoughnut = canvasDoughnut.getContext('2d');

    if (typesBarChart) typesBarChart.destroy();
    if (typesDoughnutChart) typesDoughnutChart.destroy();

    if (!stats || stats.length === 0) {
        // Clear canvases if no data
        ctxBar.clearRect(0, 0, canvasBar.width, canvasBar.height);
        ctxDoughnut.clearRect(0, 0, canvasDoughnut.width, canvasDoughnut.height);
        return;
    }

    const top5 = stats.slice(0, 5);
    const labels = top5.map(s => s.ext);
    const dataSize = top5.map(s => s.size);
    const colors = getChartColors();

    // Bar Chart (Size)
    typesBarChart = new Chart(ctxBar, {
        type: 'bar',
        data: {
            labels: labels,
            datasets: [{
                label: 'Size',
                data: dataSize,
                backgroundColor: colors.slice(0, 5),
                borderRadius: 4
            }]
        },
        options: {
            responsive: true,
            maintainAspectRatio: false,
            plugins: { legend: { display: false } },
            scales: {
                y: {
                    beginAtZero: true,
                    ticks: { callback: (value) => formatBytes(value, 0) }
                }
            }
        }
    });

    // Doughnut Chart (Size distribution)
    typesDoughnutChart = new Chart(ctxDoughnut, {
        type: 'doughnut',
        data: {
            labels: labels,
            datasets: [{
                data: dataSize,
                backgroundColor: colors.slice(0, 5),
                borderWidth: 0
            }]
        },
        options: {
            responsive: true,
            maintainAspectRatio: false,
            plugins: {
                legend: { position: 'right', labels: { boxWidth: 10 } }
            }
        }
    });
}

function renderLargestFiles(files) {
    const tbody = document.querySelector('#largestFilesTable tbody');
    if (!tbody) return;
    tbody.innerHTML = '';

    const largest = files.filter(f => !f.is_dir)
                         .sort((a, b) => b.size - a.size)
                         .slice(0, 10);

    largest.forEach(f => {
        const tr = document.createElement('tr');
        tr.innerHTML = `
            <td><i class="fas fa-file file-icon"></i> ${f.name}</td>
            <td>${formatBytes(f.size)}</td>
            <td>${new Date(f.modified * 1000).toLocaleDateString()}</td>
        `;
        tbody.appendChild(tr);
    });
}

function renderLargestDirs(files) {
    const tbody = document.querySelector('#largestDirsTable tbody');
    tbody.innerHTML = '';

    // Add "..." row if parent exists
    if (currentData && currentData.parent) {
        const tr = document.createElement('tr');
        tr.className = 'parent-dir-row';
        tr.innerHTML = `
            <td><i class="fas fa-level-up-alt"></i> ...</td>
            <td>-</td>
            <td>-</td>
        `;
        tr.style.cursor = 'pointer';
        tr.addEventListener('click', () => scan(currentData.parent));
        tbody.appendChild(tr);
    }

    const largest = files.filter(f => f.is_dir)
                         .sort((a, b) => b.size - a.size)
                         .slice(0, 10);

    largest.forEach(f => {
        const tr = document.createElement('tr');
        tr.innerHTML = `
            <td><i class="fas fa-folder folder-icon"></i> ${f.name}</td>
            <td>${formatBytes(f.size)}</td>
            <td>${f.file_count.toLocaleString()}</td>
        `;
        tr.style.cursor = 'pointer';
        tr.addEventListener('click', () => scan(f.path));
        tbody.appendChild(tr);
    });
}

function renderFileBrowser(files, totalSize) {
    const tbody = document.querySelector('#fileTable tbody');
    tbody.innerHTML = '';

    // Add "..." row if parent exists
    if (currentData && currentData.parent) {
        const tr = document.createElement('tr');
        tr.className = 'parent-dir-row';
        tr.innerHTML = `
            <td><i class="fas fa-level-up-alt"></i> ...</td>
            <td>-</td>
            <td>-</td>
            <td>-</td>
            <td>-</td>
        `;
        tr.style.cursor = 'pointer';
        tr.addEventListener('click', () => scan(currentData.parent));
        tbody.appendChild(tr);
    }

    files.forEach(file => {
        const tr = document.createElement('tr');
        const percent = totalSize > 0 ? (file.size / totalSize) * 100 : 0;
        const iconClass = file.is_dir ? 'fas fa-folder folder-icon' : 'fas fa-file file-icon';
        
        tr.innerHTML = `
            <td><i class="${iconClass}"></i> ${file.name}</td>
            <td>${formatBytes(file.size)}</td>
            <td>
                <div style="display: flex; align-items: center; gap: 10px;">
                    <span>${percent.toFixed(1)}%</span>
                    <div class="size-bar-bg">
                        <div class="size-bar-fill" style="width: ${percent}%"></div>
                    </div>
                </div>
            </td>
            <td>${file.is_dir ? file.file_count.toLocaleString() : '-'}</td>
            <td>${new Date(file.modified * 1000).toLocaleDateString()}</td>
        `;

        if (file.is_dir) {
            tr.addEventListener('click', () => scan(file.path));
        }

        tbody.appendChild(tr);
    });
}

function formatBytes(bytes, decimals = 2) {
    if (bytes === 0) return '0 Bytes';
    const k = 1024;
    const dm = decimals < 0 ? 0 : decimals;
    const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + ' ' + sizes[i];
}

function setLoading(isLoading) {
    const btn = document.getElementById('scanBtn');
    if (isLoading) {
        btn.innerHTML = '<i class="fas fa-stop"></i> Stop';
        btn.classList.add('scanning');
    } else {
        btn.innerHTML = '<i class="fas fa-search"></i> Scan';
        btn.classList.remove('scanning');
        btn.disabled = false;
    }
}

function toggleTheme() {
    const body = document.body;
    const isDark = body.getAttribute('data-theme') === 'dark';
    
    if (isDark) {
        body.removeAttribute('data-theme');
        localStorage.setItem('theme', 'light');
    } else {
        body.setAttribute('data-theme', 'dark');
        localStorage.setItem('theme', 'dark');
    }
    
    // Update logo
    /* 
    const logo = document.getElementById('appLogo');
    if (logo) {
        logo.src = isDark ? 'logo_volumetrik.png' : 'logo_volumetrik_mini.png';
    }
    */

    if (currentData) {
        renderDashboard(currentData);
    }
}

// Modal Logic (Same as before)
const modal = document.getElementById('browseModal');
const closeModal = document.querySelector('.close-modal');
const modalUpBtn = document.getElementById('modalUpBtn');
const modalPathInput = document.getElementById('modalPathInput');
const folderList = document.getElementById('folderList');
const modalSelectBtn = document.getElementById('modalSelectBtn');

let currentBrowsePath = '';

if (closeModal) {
    closeModal.onclick = () => modal.style.display = 'none';
    window.onclick = (event) => {
        if (event.target == modal) modal.style.display = 'none';
    }
}

if (modalSelectBtn) {
    modalSelectBtn.onclick = () => {
        const pathInput = document.getElementById('pathInput');
        pathInput.value = currentBrowsePath;
        modal.style.display = 'none';
        scan(currentBrowsePath);
    };
}

function openBrowseModal() {
    modal.style.display = 'block';
    loadDirectories(''); 
}

async function loadDirectories(path) {
    try {
        const response = await fetch(`${API_URL}/browse?path=${encodeURIComponent(path)}`);
        if (response.ok) {
            const data = await response.json();
            currentBrowsePath = data.current;
            modalPathInput.value = data.current;
            
            modalUpBtn.onclick = () => {
                if (data.parent) loadDirectories(data.parent);
            };
            modalUpBtn.disabled = !data.parent;

            folderList.innerHTML = '';
            data.directories.forEach(dir => {
                const li = document.createElement('li');
                li.innerHTML = `<i class="fas fa-folder"></i> ${dir}`;
                li.onclick = () => {
                    const newPath = joinPath(currentBrowsePath, dir);
                    loadDirectories(newPath);
                };
                folderList.appendChild(li);
            });
        }
    } catch (error) {
        console.error('Error browsing:', error);
    }
}

function joinPath(base, part) {
    if (base.endsWith('/') || base.endsWith('\\')) {
        return base + part;
    }
    const separator = base.includes('\\') ? '\\' : '/';
    return base + separator + part;
}

// --- Settings Modal Logic ---
const settingsBtn = document.getElementById('settingsBtn');
const settingsModal = document.getElementById('settingsModal');
const closeSettings = document.querySelector('.close-settings');
const closeSettingsBtn = document.querySelector('.close-settings-btn');
const saveSettingsBtn = document.getElementById('saveSettingsBtn');
const addPathBtn = document.getElementById('addPathBtn');
const newMonitorPathInput = document.getElementById('newMonitorPath');
const monitorPathList = document.getElementById('monitorPathList');
const languageSelect = document.getElementById('languageSelect');
const paletteSelect = document.getElementById('paletteSelect');
const resetLayoutBtn = document.getElementById('resetLayoutBtn');

let monitoredPaths = [];
let initialLanguage = 'en';

if (settingsBtn) {
    settingsBtn.onclick = () => {
        settingsModal.style.display = 'block';
        loadSettings();
        // Load current palette into select
        if (paletteSelect) {
            paletteSelect.value = localStorage.getItem('palette') || 'default';
        }
    };
}

if (resetLayoutBtn) {
    resetLayoutBtn.onclick = async () => {
        if (confirm(getTranslation('reset_layout') + '?')) {
            try {
                await fetch(`${API_URL}/layout`, {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify([]) // Empty array to reset
                });
                showToast(getTranslation('layout_reset'), 'success');
                setTimeout(() => location.reload(), 1000);
            } catch (e) {
                console.error('Error resetting layout:', e);
            }
        }
    };
}

if (addPathBtn) {
    addPathBtn.onclick = () => {
        const path = newMonitorPathInput.value.trim();
        const type = document.getElementById('newMonitorType').value;
        const value = parseFloat(document.getElementById('newMonitorValue').value);

        if (path && !isNaN(value)) {
            // Check if path already exists
            const exists = monitoredPaths.some(p => p.path === path);
            if (!exists) {
                monitoredPaths.push({
                    path: path,
                    threshold_type: type,
                    threshold_value: value
                });
                renderMonitoredPaths();
                newMonitorPathInput.value = '';
                document.getElementById('newMonitorValue').value = '';
            } else {
                showToast(getTranslation('path_already_monitored'), 'error');
            }
        } else {
            showToast(getTranslation('invalid_path_threshold'), 'error');
        }
    };
}

function renderMonitoredPaths() {
    monitorPathList.innerHTML = '';
    monitoredPaths.forEach((item, index) => {
        const li = document.createElement('li');
        li.className = 'path-item';
        li.innerHTML = `
            <div style="display: flex; flex-direction: column; gap: 2px;">
                <span style="font-weight: bold;">${item.path}</span>
                <span style="font-size: 0.8em; color: #7f8c8d;">${item.threshold_type === 'MaxUsed' ? 'Max Used' : 'Min Remaining'}: ${item.threshold_value} GB</span>
            </div>
            <button type="button" class="remove-path-btn" onclick="removePath(${index})"><i class="fas fa-trash"></i></button>
        `;
        monitorPathList.appendChild(li);
    });
}

window.removePath = function(index) {
    monitoredPaths.splice(index, 1);
    renderMonitoredPaths();
};

if (closeSettings) {
    closeSettings.onclick = () => settingsModal.style.display = 'none';
}

if (closeSettingsBtn) {
    closeSettingsBtn.onclick = () => settingsModal.style.display = 'none';
}

// Tab Switching Logic
document.querySelectorAll('.settings-tab').forEach(tab => {
    tab.addEventListener('click', () => {
        // Remove active class from all tabs and panels
        document.querySelectorAll('.settings-tab').forEach(t => t.classList.remove('active'));
        document.querySelectorAll('.settings-panel').forEach(p => p.classList.remove('active'));

        // Add active class to clicked tab
        tab.classList.add('active');

        // Show corresponding panel
        const tabId = tab.getAttribute('data-tab');
        document.getElementById(`tab-${tabId}`).classList.add('active');
    });
});

// Close modal when clicking outside
window.addEventListener('click', (event) => {
    if (event.target == settingsModal) {
        settingsModal.style.display = 'none';
    }
});

if (saveSettingsBtn) {
    saveSettingsBtn.onclick = saveSettings;
}

if (languageSelect) {
    languageSelect.addEventListener('change', (e) => {
        updateLanguage(e.target.value);
    });
}

if (paletteSelect) {
    paletteSelect.addEventListener('change', (e) => {
        applyPalette(e.target.value);
    });
}

async function loadSettings() {
    try {
        const response = await fetch(`${API_URL}/settings`);
        if (response.ok) {
            const settings = await response.json();
            
            // General
            initialLanguage = settings.language || 'en';
            if (languageSelect) {
                languageSelect.value = initialLanguage;
            }
            updateLanguage(initialLanguage);

            // Monitoring
            document.getElementById('monitorEnabled').checked = settings.monitoring.enabled;
            
            // Handle legacy single path or new array
            if (settings.monitoring.paths) {
                // Check if it's the old string array or new object array
                if (settings.monitoring.paths.length > 0 && typeof settings.monitoring.paths[0] === 'string') {
                    // Convert old format to new
                    monitoredPaths = settings.monitoring.paths.map(p => ({
                        path: p,
                        threshold_type: 'MaxUsed',
                        threshold_value: settings.monitoring.threshold_gb || 100
                    }));
                } else {
                    monitoredPaths = settings.monitoring.paths;
                }
            } else {
                monitoredPaths = [];
            }
            renderMonitoredPaths();
            
            // document.getElementById('monitorThreshold').value = settings.monitoring.threshold_gb; // Removed global threshold
            document.getElementById('monitorInterval').value = settings.monitoring.check_interval_minutes;

            // Alerts
            document.getElementById('alertsEnabled').checked = settings.alerts.enabled;
            document.getElementById('customAlertMessage').value = settings.alerts.custom_message || '';
            document.getElementById('telegramToken').value = settings.alerts.telegram_bot_token || '';
            document.getElementById('telegramChatId').value = settings.alerts.telegram_chat_id || '';
            document.getElementById('webhookUrl').value = settings.alerts.webhook_url || '';
            
            // New Services
            document.getElementById('pushoverUserKey').value = settings.alerts.pushover_user_key || '';
            document.getElementById('pushoverApiToken').value = settings.alerts.pushover_api_token || '';
            document.getElementById('gotifyUrl').value = settings.alerts.gotify_url || '';
            document.getElementById('gotifyToken').value = settings.alerts.gotify_token || '';
            document.getElementById('ntfyUrl').value = settings.alerts.ntfy_url || '';
            document.getElementById('ntfyToken').value = settings.alerts.ntfy_token || '';
            document.getElementById('slackUrl').value = settings.alerts.slack_webhook_url || '';
            document.getElementById('discordUrl').value = settings.alerts.discord_webhook_url || '';
            document.getElementById('teamsUrl').value = settings.alerts.teams_webhook_url || '';
        }
    } catch (error) {
        console.error('Error loading settings:', error);
    }
}

async function saveSettings() {
    console.log("Saving settings...");
    // Capture current layout to preserve it
    const layout = [];
    if (grid && grid.engine) {
        grid.engine.nodes.forEach(node => {
            layout.push({
                id: node.el.id,
                x: node.x,
                y: node.y,
                w: node.w,
                h: node.h
            });
        });
    }

    const currentLanguage = document.getElementById('languageSelect').value;
    const currentPalette = document.getElementById('paletteSelect').value;

    const settings = {
        language: currentLanguage,
        monitoring: {
            enabled: document.getElementById('monitorEnabled').checked,
            paths: monitoredPaths,
            // threshold_gb: parseFloat(document.getElementById('monitorThreshold').value) || 100.0, // Removed
            check_interval_minutes: parseInt(document.getElementById('monitorInterval').value) || 60
        },
        alerts: {
            enabled: document.getElementById('alertsEnabled').checked,
            custom_message: document.getElementById('customAlertMessage').value || null,
            telegram_bot_token: document.getElementById('telegramToken').value || null,
            telegram_chat_id: document.getElementById('telegramChatId').value || null,
            webhook_url: document.getElementById('webhookUrl').value || null,
            
            // New Services
            pushover_user_key: document.getElementById('pushoverUserKey').value || null,
            pushover_api_token: document.getElementById('pushoverApiToken').value || null,
            gotify_url: document.getElementById('gotifyUrl').value || null,
            gotify_token: document.getElementById('gotifyToken').value || null,
            ntfy_url: document.getElementById('ntfyUrl').value || null,
            ntfy_token: document.getElementById('ntfyToken').value || null,
            slack_webhook_url: document.getElementById('slackUrl').value || null,
            discord_webhook_url: document.getElementById('discordUrl').value || null,
            teams_webhook_url: document.getElementById('teamsUrl').value || null
        },
        layout: layout.length > 0 ? layout : null
    };

    try {
        const response = await fetch(`${API_URL}/settings`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(settings)
        });

        if (response.ok) {
            showToast(getTranslation('settings_saved_success'), 'success');
            settingsModal.style.display = 'none';
            
            if (currentLanguage !== initialLanguage) {
                updateLanguage(currentLanguage);
            }
            initialLanguage = currentLanguage;
            
            // Apply palette
            applyPalette(currentPalette);
        } else {
            const errorText = await response.text();
            console.error('Failed to save settings:', response.status, response.statusText, errorText);
            showToast(getTranslation('settings_save_failed') + errorText, 'error');
        }
    } catch (error) {
        console.error('Error saving settings:', error);
        showToast(getTranslation('settings_save_failed'), 'error');
    }
}

async function saveLayout() {
    const layout = [];
    if (grid && grid.engine) {
        grid.engine.nodes.forEach(node => {
            layout.push({
                id: node.el.id,
                x: node.x,
                y: node.y,
                w: node.w,
                h: node.h
            });
        });
    }
    
    try {
        await fetch(`${API_URL}/layout`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(layout)
        });
        showToast(getTranslation('layout_saved'), 'success');
    } catch (e) {
        console.error('Error saving layout:', e);
        showToast(getTranslation('error_saving_layout') + e.message, 'error');
    }
}

function showToast(message, type = 'success') {
    const toast = document.getElementById('toast');
    toast.textContent = message;
    toast.className = `toast show ${type}`;
    setTimeout(() => {
        toast.className = toast.className.replace('show', '');
    }, 3000);
}

async function loadLayout() {
    try {
        const response = await fetch(`${API_URL}/settings`);
        if (response.ok) {
            const settings = await response.json();
            if (settings.layout) {
                const items = settings.layout;
                grid.batchUpdate();
                items.forEach(item => {
                    const el = document.getElementById(item.id);
                    if (el) {
                        grid.update(el, { x: item.x, y: item.y, w: item.w, h: item.h });
                    }
                });
                grid.commit();
            }
        }
    } catch (e) {
        console.error('Error loading layout:', e);
        showToast(getTranslation('error_loading_layout') + e.message, 'error');
    }
}
