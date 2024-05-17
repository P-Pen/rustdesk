lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "Статус"),
        ("Your Desktop", "Ваша стільниця"),
        ("desk_tip", "Ваша стільниця доступна з цим ідентифікатором і паролем"),
        ("Password", "Пароль"),
        ("Ready", "Готово"),
        ("Established", "Встановлено"),
        ("connecting_status", "Підключення до мережі PNDesk..."),
        ("Enable service", "Включити службу"),
        ("Start service", "Запустити службу"),
        ("Service is running", "Служба працює"),
        ("Service is not running", "Служба не запущена"),
        ("not_ready_status", "Не готово. Будь ласка, перевірте ваше підключення"),
        ("Control Remote Desktop", "Керування віддаленою стільницею"),
        ("Transfer file", "Надіслати файл"),
        ("Connect", "Підключитися"),
        ("Recent sessions", "Нещодавні сеанси"),
        ("Address book", "Адресна книга"),
        ("Confirmation", "Підтвердження"),
        ("TCP tunneling", "TCP-тунелювання"),
        ("Remove", "Видалити"),
        ("Refresh random password", "Оновити випадковий пароль"),
        ("Set your own password", "Встановити свій пароль"),
        ("Enable keyboard/mouse", "Увімкнути клавіатуру/мишу"),
        ("Enable clipboard", "Увімкнути буфер обміну"),
        ("Enable file transfer", "Увімкнути передачу файлів"),
        ("Enable TCP tunneling", "Увімкнути тунелювання TCP"),
        ("IP Whitelisting", "Список дозволених IP-адрес"),
        ("ID/Relay Server", "ID/Сервер ретрансляції"),
        ("Import server config", "Імпортувати конфігурацію сервера"),
        ("Export Server Config", "Експортувати конфігурацію сервера"),
        ("Import server configuration successfully", "Конфігурацію сервера успішно імпортовано"),
        ("Export server configuration successfully", "Конфігурацію сервера успішно експортовано"),
        ("Invalid server configuration", "Неправильна конфігурація сервера"),
        ("Clipboard is empty", "Буфер обміну порожній"),
        ("Stop service", "Зупинити службу"),
        ("Change ID", "Змінити ID"),
        ("Your new ID", "Ваш новий ID"),
        ("length %min% to %max%", "від %min% до %max% символів"),
        ("starts with a letter", "починається з літери"),
        ("allowed characters", "дозволені символи"),
        ("id_change_tip", "Допускаються лише символи a-z, A-Z, 0-9 і _ (підкреслення). Першою повинна бути літера a-z, A-Z. В межах від 6 до 16 символів"),
        ("Website", "Веб-сайт"),
        ("About", "Про PNDesk"),
        ("Slogan_tip", "Створено з душею в цьому хаотичному світі!"),
        ("Privacy Statement", "Декларація про конфіденційність"),
        ("Mute", "Вимкнути звук"),
        ("Build Date", "Дата збірки"),
        ("Version", "Версія"),
        ("Home", "Домівка"),
        ("Audio Input", "Аудіовхід"),
        ("Enhancements", "Покращення"),
        ("Hardware Codec", "Апаратний кодек"),
        ("Adaptive bitrate", "Адаптивна швидкість потоку"),
        ("ID Server", "ID-сервер"),
        ("Relay Server", "Сервер ретрансляції"),
        ("API Server", "API-сервер"),
        ("invalid_http", "Повинна починатися з http:// або https://"),
        ("Invalid IP", "Неправильна IP-адреса"),
        ("Invalid format", "Неправильний формат"),
        ("server_not_support", "Наразі не підтримується сервером"),
        ("Not available", "Недоступно"),
        ("Too frequent", "Занадто часто"),
        ("Cancel", "Скасувати"),
        ("Skip", "Пропустити"),
        ("Close", "Закрити"),
        ("Retry", "Спробувати знову"),
        ("OK", "OK"),
        ("Password Required", "Потрібен пароль"),
        ("Please enter your password", "Будь ласка, введіть ваш пароль"),
        ("Remember password", "Запамʼятати пароль"),
        ("Wrong Password", "Неправильний пароль"),
        ("Do you want to enter again?", "Бажаєте увійти знову?"),
        ("Connection Error", "Помилка підключення"),
        ("Error", "Помилка"),
        ("Reset by the peer", "Віддалений пристрій скинув підключення"),
        ("Connecting...", "Підключення..."),
        ("Connection in progress. Please wait.", "Виконується підключення. Будь ласка, зачекайте."),
        ("Please try 1 minute later", "Спробуйте через 1 хвилину"),
        ("Login Error", "Помилка входу"),
        ("Successful", "Операція успішна"),
        ("Connected, waiting for image...", "Підключено, очікування зображення..."),
        ("Name", "Імʼя"),
        ("Type", "Тип"),
        ("Modified", "Змінено"),
        ("Size", "Розмір"),
        ("Show Hidden Files", "Показати приховані файли"),
        ("Receive", "Отримати"),
        ("Send", "Надіслати"),
        ("Refresh File", "Оновити файл"),
        ("Local", "Локальний"),
        ("Remote", "Віддалений"),
        ("Remote Computer", "Віддалений компʼютер"),
        ("Local Computer", "Локальний компʼютер"),
        ("Confirm Delete", "Підтвердити видалення"),
        ("Delete", "Видалити"),
        ("Properties", "Властивості"),
        ("Multi Select", "Багатоелементний вибір"),
        ("Select All", "Вибрати все"),
        ("Unselect All", "Скасувати вибір"),
        ("Empty Directory", "Порожня тека"),
        ("Not an empty directory", "Тека не порожня"),
        ("Are you sure you want to delete this file?", "Ви впевнені, що хочете видалити цей файл?"),
        ("Are you sure you want to delete this empty directory?", "Ви впевнені, що хочете видалити порожню теку?"),
        ("Are you sure you want to delete the file of this directory?", "Ви впевнені, що хочете видалити файл із цієї теки?"),
        ("Do this for all conflicts", "Це стосується всіх конфліктів"),
        ("This is irreversible!", "Це незворотна дія!"),
        ("Deleting", "Видалення"),
        ("files", "файли"),
        ("Waiting", "Очікування"),
        ("Finished", "Завершено"),
        ("Speed", "Швидкість"),
        ("Custom Image Quality", "Користувацька якість зображення"),
        ("Privacy mode", "Режим конфіденційності"),
        ("Block user input", "Блокувати користувацьке введення"),
        ("Unblock user input", "Розблокувати користувацьке введення"),
        ("Adjust Window", "Налаштувати вікно"),
        ("Original", "Оригінал"),
        ("Shrink", "Зменшити"),
        ("Stretch", "Розтягнути"),
        ("Scrollbar", "Смуга прокрутки"),
        ("ScrollAuto", "Автоматична прокрутка"),
        ("Good image quality", "Хороша якість зображення"),
        ("Balanced", "Збалансована"),
        ("Optimize reaction time", "Оптимізувати час реакції"),
        ("Custom", "Користувацька"),
        ("Show remote cursor", "Показати віддалений вказівник"),
        ("Show quality monitor", "Показати якість"),
        ("Disable clipboard", "Вимкнути буфер обміну"),
        ("Lock after session end", "Блокування після завершення сеансу"),
        ("Insert", "Вставити"),
        ("Insert Lock", "Встановити замок"),
        ("Refresh", "Оновити"),
        ("ID does not exist", "ID не існує"),
        ("Failed to connect to rendezvous server", "Не вдалося підключитися до сервера рандеву"),
        ("Please try later", "Будь ласка, спробуйте пізніше"),
        ("Remote desktop is offline", "Віддалена стільниця не в мережі"),
        ("Key mismatch", "Невідповідність ключів"),
        ("Timeout", "Час очікування"),
        ("Failed to connect to relay server", "Не вдалося підключитися до сервера ретрансляції"),
        ("Failed to connect via rendezvous server", "Не вдалося підключитися через сервер рандеву"),
        ("Failed to connect via relay server", "Не вдалося підключитися через сервер ретрансляції"),
        ("Failed to make direct connection to remote desktop", "Не вдалося встановити пряме підключення до віддаленої стільниці"),
        ("Set Password", "Встановити пароль"),
        ("OS Password", "Пароль ОС"),
        ("install_tip", "Через UAC в деяких випадках PNDesk може працювати некоректно на віддаленому вузлі. Щоб уникнути UAC, натисніть кнопку нижче для встановлення PNDesk в системі"),
        ("Click to upgrade", "Натисніть, щоб перевірити наявність оновлень"),
        ("Click to download", "Натисніть, щоб завантажити"),
        ("Click to update", "Натисніть, щоб оновити"),
        ("Configure", "Налаштувати"),
        ("config_acc", "Для віддаленого керування вашою стільницею, вам необхідно надати PNDesk дозволи \"Доступності\""),
        ("config_screen", "Для віддаленого доступу до вашої стільниці,вам необхідно надати PNDesk дозволи на \"Запис екрану\""),
        ("Installing ...", "Встановлюється..."),
        ("Install", "Встановити"),
        ("Installation", "Встановлення"),
        ("Installation Path", "Шлях встановлення"),
        ("Create start menu shortcuts", "Створити ярлики меню \"Пуск\""),
        ("Create desktop icon", "Створити значок на стільниці"),
        ("agreement_tip", "Починаючи встановлення, ви приймаєте умови ліцензійної угоди"),
        ("Accept and Install", "Прийняти та встановити"),
        ("End-user license agreement", "Ліцензійна угода з кінцевим користувачем"),
        ("Generating ...", "Генерація..."),
        ("Your installation is lower version.", "У вас встановлена більш рання версія"),
        ("not_close_tcp_tip", "Не закривайте це вікно під час використання тунелю"),
        ("Listening ...", "Очікуємо ..."),
        ("Remote Host", "Віддалена машина"),
        ("Remote Port", "Віддалений порт"),
        ("Action", "Дія"),
        ("Add", "Додати"),
        ("Local Port", "Локальний порт"),
        ("Local Address", "Локальна адреса"),
        ("Change Local Port", "Змінити локальний порт"),
        ("setup_server_tip", "Для пришвидшення зʼєднання, будь ласка, налаштуйте власний сервер"),
        ("Too short, at least 6 characters.", "Занадто коротко, мінімум 6 символів"),
        ("The confirmation is not identical.", "Підтвердження не збігається"),
        ("Permissions", "Дозволи"),
        ("Accept", "Прийняти"),
        ("Dismiss", "Відхилити"),
        ("Disconnect", "Відʼєднати"),
        ("Enable file copy and paste", "Дозволити копіювання та вставку файлів"),
        ("Connected", "Підключено"),
        ("Direct and encrypted connection", "Пряме та зашифроване підключення"),
        ("Relayed and encrypted connection", "Ретрансльоване та зашифроване підключення"),
        ("Direct and unencrypted connection", "Пряме та незашифроване підключення"),
        ("Relayed and unencrypted connection", "Ретрансльоване та незашифроване підключення"),
        ("Enter Remote ID", "Введіть віддалений ID"),
        ("Enter your password", "Введіть пароль"),
        ("Logging in...", "Вхід..."),
        ("Enable RDP session sharing", "Включити загальний доступ до сеансу RDP"),
        ("Auto Login", "Автоматичний вхід (дійсний, тільки якщо ви встановили \"Завершення користувацького сеансу після завершення віддаленого підключення\")"),
        ("Enable direct IP access", "Увімкнути прямий IP-доступ"),
        ("Rename", "Перейменувати"),
        ("Space", "Місце"),
        ("Create desktop shortcut", "Створити ярлик на стільниці"),
        ("Change Path", "Змінити шлях"),
        ("Create Folder", "Створити теку"),
        ("Please enter the folder name", "Будь ласка, введіть назву для теки"),
        ("Fix it", "Виправити"),
        ("Warning", "Попередження"),
        ("Login screen using Wayland is not supported", "Вхід в систему з використанням Wayland не підтримується"),
        ("Reboot required", "Потрібне перезавантаження"),
        ("Unsupported display server", "Графічний сервер не підтримується"),
        ("x11 expected", "Очікується X11"),
        ("Port", "Порт"),
        ("Settings", "Налаштування"),
        ("Username", "Імʼя користувача"),
        ("Invalid port", "Неправильний порт"),
        ("Closed manually by the peer", "Завершено вручну з боку віддаленого пристрою"),
        ("Enable remote configuration modification", "Дозволити віддалену зміну конфігурації"),
        ("Run without install", "Запустити без встановлення"),
        ("Connect via relay", "Підключитися через ретранслятор"),
        ("Always connect via relay", "Завжди підключатися через ретранслятор"),
        ("whitelist_tip", "Тільки IP-адреси з білого списку можуть отримати доступ до мене"),
        ("Login", "Увійти"),
        ("Verify", "Підтвердити"),
        ("Remember me", "Запамʼятати мене"),
        ("Trust this device", "Довірений пристрій"),
        ("Verification code", "Код підтвердження"),
        ("verification_tip", "Виявлено новий пристрій, код підтвердження надіслано на зареєстровану email-адресу, введіть код підтвердження для продовження авторизації."),
        ("Logout", "Вийти"),
        ("Tags", "Теги"),
        ("Search ID", "Пошук за ID"),
        ("whitelist_sep", "Розділені комою, крапкою з комою, пробілом або новим рядком"),
        ("Add ID", "Додати ID"),
        ("Add Tag", "Додати ключове слово"),
        ("Unselect all tags", "Скасувати вибір усіх тегів"),
        ("Network error", "Помилка мережі"),
        ("Username missed", "Імʼя користувача відсутнє"),
        ("Password missed", "Пароль відсутній"),
        ("Wrong credentials", "Неправильні дані"),
        ("The verification code is incorrect or has expired", "Код підтвердження некоректний або протермінований"),
        ("Edit Tag", "Редагувати тег"),
        ("Forget Password", "Не зберігати пароль"),
        ("Favorites", "Вибране"),
        ("Add to Favorites", "Додати в обране"),
        ("Remove from Favorites", "Видалити з обраного"),
        ("Empty", "Пусто"),
        ("Invalid folder name", "Неприпустима назва теки"),
        ("Socks5 Proxy", "Проксі-сервер Socks5"),
        ("Socks5/Http(s) Proxy", "Проксі-сервер Socks5/Http(s)"),
        ("Discovered", "Знайдено"),
        ("install_daemon_tip", "Для запуску під час завантаження, вам необхідно встановити системну службу"),
        ("Remote ID", "Віддалений ідентифікатор"),
        ("Paste", "Вставити"),
        ("Paste here?", "Вставити сюди?"),
        ("Are you sure to close the connection?", "Ви впевнені, що хочете завершити підключення?"),
        ("Download new version", "Завантажити нову версію"),
        ("Touch mode", "Сенсорний режим"),
        ("Mouse mode", "Режим миші"),
        ("One-Finger Tap", "Дотик одним пальцем"),
        ("Left Mouse", "Ліва кнопка миші"),
        ("One-Long Tap", "Одне довге натискання пальцем"),
        ("Two-Finger Tap", "Дотик двома пальцями"),
        ("Right Mouse", "Права миша"),
        ("One-Finger Move", "Рух одним пальцем"),
        ("Double Tap & Move", "Подвійне натискання та переміщення"),
        ("Mouse Drag", "Перетягування мишею"),
        ("Three-Finger vertically", "Трьома пальцями по вертикалі"),
        ("Mouse Wheel", "Коліщатко миші"),
        ("Two-Finger Move", "Рух двома пальцями"),
        ("Canvas Move", "Переміщення полотна"),
        ("Pinch to Zoom", "Стисніть, щоб збільшити"),
        ("Canvas Zoom", "Масштаб полотна"),
        ("Reset canvas", "Відновлення полотна"),
        ("No permission of file transfer", "Немає дозволу на передачу файлів"),
        ("Note", "Примітка"),
        ("Connection", "Підключення"),
        ("Share Screen", "Поділитися екраном"),
        ("Chat", "Чат"),
        ("Total", "Всього"),
        ("items", "елементи"),
        ("Selected", "Обрано"),
        ("Screen Capture", "Захоплення екрана"),
        ("Input Control", "Керування введенням"),
        ("Audio Capture", "Захоплення аудіо"),
        ("File Connection", "Файлове підключення"),
        ("Screen Connection", "Підключення екрана"),
        ("Do you accept?", "Ви згодні?"),
        ("Open System Setting", "Відкрити налаштування системи"),
        ("How to get Android input permission?", "Як отримати дозвіл на введення Android?"),
        ("android_input_permission_tip1", "Для того, щоб віддалений пристрій міг керувати вашим Android-пристроєм за допомогою миші або торкання, вам необхідно дозволити PNDesk використовувати службу \"Спеціальні можливості\"."),
        ("android_input_permission_tip2", "Перейдіть на наступну сторінку системних налаштувань, знайдіть та увійдіть у [Встановлені служби], увімкніть службу [PNDesk Input]."),
        ("android_new_connection_tip", "Отримано новий запит на керування вашим поточним пристроєм."),
        ("android_service_will_start_tip", "Увімкнення захоплення екрана автоматично запускає службу, дозволяючи іншим пристроям запитувати підключення до вашого пристрою."),
        ("android_stop_service_tip", "Зупинка служби автоматично завершить всі встановлені зʼєднання."),
        ("android_version_audio_tip", "Поточна версія Android не підтримує захоплення звуку, оновіть її до Android 10 або вище."),
        ("android_start_service_tip", "Натисніть [Запустити службу] або увімкніть дозвіл на [Захоплення екрана], щоб запустити службу спільного доступу до екрана."),
        ("android_permission_may_not_change_tip", "Дозволи для встановлених зʼєднань можуть не змінитися миттєво аж до перепідключення."),
        ("Account", "Акаунт"),
        ("Overwrite", "Перезаписати"),
        ("This file exists, skip or overwrite this file?", "Цей файл існує, пропустити чи перезаписати файл?"),
        ("Quit", "Вийти"),
        ("Help", "Допомога"),
        ("Failed", "Не вдалося"),
        ("Succeeded", "Успішно"),
        ("Someone turns on privacy mode, exit", "Хтось вмикає режим конфіденційності, вихід"),
        ("Unsupported", "Не підтримується"),
        ("Peer denied", "Відхилено віддаленим пристроєм"),
        ("Please install plugins", "Будь ласка, встановіть плагіни"),
        ("Peer exit", "Вийти з віддаленого пристрою"),
        ("Failed to turn off", "Не вдалося вимкнути"),
        ("Turned off", "Вимкнений"),
        ("Language", "Мова"),
        ("Keep PNDesk background service", "Зберегти фонову службу PNDesk"),
        ("Ignore Battery Optimizations", "Ігнорувати оптимізації батареї"),
        ("android_open_battery_optimizations_tip", "Перейдіть на наступну сторінку налаштувань"),
        ("Start on boot", "Автозапуск"),
        ("Start the screen sharing service on boot, requires special permissions", "Запустити службу службу спільного доступу до екрана під час завантаження, потребує спеціальних дозволів"),
        ("Connection not allowed", "Підключення не дозволено"),
        ("Legacy mode", "Застарілий режим"),
        ("Map mode", "Режим карти"),
        ("Translate mode", "Режим перекладу"),
        ("Use permanent password", "Використовувати постійний пароль"),
        ("Use both passwords", "Використовувати обидва паролі"),
        ("Set permanent password", "Встановити постійний пароль"),
        ("Enable remote restart", "Увімкнути віддалений перезапуск"),
        ("Restart remote device", "Перезапустити віддалений пристрій"),
        ("Are you sure you want to restart", "Ви впевнені, що хочете виконати перезапуск?"),
        ("Restarting remote device", "Перезапуск віддаленого пристрою"),
        ("remote_restarting_tip", "Віддалений пристрій перезапускається. Будь ласка, закрийте це повідомлення та через деякий час перепідключіться, використовуючи постійний пароль."),
        ("Copied", "Скопійовано"),
        ("Exit Fullscreen", "Вийти з повноекранного режиму"),
        ("Fullscreen", "Повноекранний"),
        ("Mobile Actions", "Мобільні дії"),
        ("Select Monitor", "Виберіть монітор"),
        ("Control Actions", "Дії для керування"),
        ("Display Settings", "Налаштування дисплею"),
        ("Ratio", "Співвідношення"),
        ("Image Quality", "Якість зображення"),
        ("Scroll Style", "Стиль прокрутки"),
        ("Show Toolbar", "Показати панель інструментів"),
        ("Hide Toolbar", "Приховати панель інструментів"),
        ("Direct Connection", "Пряме підключення"),
        ("Relay Connection", "Ретрансльоване підключення"),
        ("Secure Connection", "Безпечне підключення"),
        ("Insecure Connection", "Небезпечне підключення"),
        ("Scale original", "Оригінал масштабу"),
        ("Scale adaptive", "Масштаб адаптивний"),
        ("General", "Загальні"),
        ("Security", "Безпека"),
        ("Theme", "Тема"),
        ("Dark Theme", "Темна тема"),
        ("Light Theme", "Світла тема"),
        ("Dark", "Темна"),
        ("Light", "Світла"),
        ("Follow System", "Як в системі"),
        ("Enable hardware codec", "Увімкнути апаратний кодек"),
        ("Unlock Security Settings", "Розблокувати налаштування безпеки"),
        ("Enable audio", "Увімкнути аудіо"),
        ("Unlock Network Settings", "Розблокувати мережеві налаштування"),
        ("Server", "Сервер"),
        ("Direct IP Access", "Прямий IP доступ"),
        ("Proxy", "Проксі"),
        ("Apply", "Застосувати"),
        ("Disconnect all devices?", "Відʼєднати всі прилади?"),
        ("Clear", "Очистити"),
        ("Audio Input Device", "Пристрій введення звуку"),
        ("Use IP Whitelisting", "Використовувати білий список IP"),
        ("Network", "Мережа"),
        ("Pin Toolbar", "Закріпити панель інструментів"),
        ("Unpin Toolbar", "Відкріпити панель інструментів"),
        ("Recording", "Запис"),
        ("Directory", "Директорія"),
        ("Automatically record incoming sessions", "Автоматично записувати вхідні сеанси"),
        ("Change", "Змінити"),
        ("Start session recording", "Розпочати запис сеансу"),
        ("Stop session recording", "Закінчити запис сеансу"),
        ("Enable recording session", "Увімкнути запис сеансу"),
        ("Enable LAN discovery", "Увімкнути пошук локальної мережі"),
        ("Deny LAN discovery", "Заборонити виявлення локальної мережі"),
        ("Write a message", "Написати повідомлення"),
        ("Prompt", "Підказка"),
        ("Please wait for confirmation of UAC...", "Будь ласка, зачекайте підтвердження UAC..."),
        ("elevated_foreground_window_tip", "Поточне вікно віддаленої стільниці потребує розширених прав для роботи, тому наразі неможливо використати мишу та клавіатуру. Ви можете запропонувати віддаленому користувачу згорнути поточне вікно чи натиснути кнопку розширення прав у вікні керування підключеннями. Для уникнення цієї проблеми, рекомендується встановити програму на віддаленому пристрої"),
        ("Disconnected", "Відʼєднано"),
        ("Other", "Інше"),
        ("Confirm before closing multiple tabs", "Підтверджувати перед закриттям кількох вкладок"),
        ("Keyboard Settings", "Налаштування клавіатури"),
        ("Full Access", "Повний доступ"),
        ("Screen Share", "Демонстрація екрану"),
        ("Wayland requires Ubuntu 21.04 or higher version.", "Wayland потребує Ubuntu 21.04 або новішої версії."),
        ("Wayland requires higher version of linux distro. Please try X11 desktop or change your OS.", "Для Wayland потрібна новіша версія дистрибутива Linux. Будь ласка, спробуйте стільницю на X11 або змініть свою ОС."),
        ("JumpLink", "Перегляд"),
        ("Please Select the screen to be shared(Operate on the peer side).", "Будь ласка, виберіть екран, до якого потрібно надати доступ (на віддаленому пристрої)."),
        ("Show PNDesk", "Показати PNDesk"),
        ("This PC", "Цей ПК"),
        ("or", "чи"),
        ("Continue with", "Продовжити з"),
        ("Elevate", "Розширення прав"),
        ("Zoom cursor", "Збільшити вказівник"),
        ("Accept sessions via password", "Підтверджувати сеанси паролем"),
        ("Accept sessions via click", "Підтверджувати сеанси натисканням"),
        ("Accept sessions via both", "Підтверджувати сеанси обома способами"),
        ("Please wait for the remote side to accept your session request...", "Буль ласка, зачекайте, поки віддалена сторона підтвердить запит на сеанс..."),
        ("One-time Password", "Одноразовий пароль"),
        ("Use one-time password", "Використати одноразовий пароль"),
        ("One-time password length", "Довжина одноразового пароля"),
        ("Request access to your device", "Дати запит щодо доступ до свого пристрою"),
        ("Hide connection management window", "Приховати вікно керування підключеннями"),
        ("hide_cm_tip", "Дозволено приховати лише якщо сеанс підтверджується постійним паролем"),
        ("wayland_experiment_tip", "Підтримка Wayland на експериментальній стадії, будь ласка, використовуйте X11, якщо необхідний автоматичний доступ."),
        ("Right click to select tabs", "Правий клік для вибору вкладки"),
        ("Skipped", "Пропущено"),
        ("Add to address book", "Додати IP до Адресної книги"),
        ("Group", "Група"),
        ("Search", "Пошук"),
        ("Closed manually by web console", "Закрито вручну з веб-консолі"),
        ("Local keyboard type", "Тип локальної клавіатури"),
        ("Select local keyboard type", "Оберіть тип локальної клавіатури"),
        ("software_render_tip", "Якщо ви використовуєте відеокарту Nvidia на Linux, і віддалене вікно закривається відразу після підключення, то перехід на вільний драйвер Nouveau та увімкнення програмного рендерингу може допомогти. Для застосування змін необхідно перезапустити програму."),
        ("Always use software rendering", "Завжди використовувати програмний рендеринг"),
        ("config_input", "Для віддаленого керування віддаленою стільницею з клавіатури, вам необхідно надати PNDesk дозволи на \"Відстеження введення\""),
        ("config_microphone", "Для можливості віддаленої розмови, вам необхідно надати PNDesk дозвіл на \"Запис аудіо\""),
        ("request_elevation_tip", "Ви також можете надіслати запит на розширення прав, в разі присутності особи з віддаленого боку."),
        ("Wait", "Зачекайте"),
        ("Elevation Error", "Невдала спроба розширення прав"),
        ("Ask the remote user for authentication", "Попросіть віддаленого користувача пройти автентифікацію"),
        ("Choose this if the remote account is administrator", "Виберіть це, якщо віддалений обліковий запис є адміністративним"),
        ("Transmit the username and password of administrator", "Передайте імʼя користувача та пароль адміністратора"),
        ("still_click_uac_tip", "Досі необхідне підтвердження UAC з боку віддаленого користувача"),
        ("Request Elevation", "Запит на розширення прав"),
        ("wait_accept_uac_tip", "Будь ласка, очікуйте підтвердження діалогу UAC з боку віддаленого користувача."),
        ("Elevate successfully", "Успішне розширення прав"),
        ("uppercase", "верхній регістр"),
        ("lowercase", "нижній регістр"),
        ("digit", "цифра"),
        ("special character", "спецсимвол"),
        ("length>=8", "довжина>=8"),
        ("Weak", "Слабкий"),
        ("Medium", "Середній"),
        ("Strong", "Сильний"),
        ("Switch Sides", "Поміняти місцями"),
        ("Please confirm if you want to share your desktop?", "Будь ласка, підтвердіть дозвіл на спільне використання стільниці"),
        ("Display", "Екран"),
        ("Default View Style", "Типовий стиль перегляду"),
        ("Default Scroll Style", "Типовий стиль гортання"),
        ("Default Image Quality", "Типова якість зображення"),
        ("Default Codec", "Типовий кодек"),
        ("Bitrate", "Бітрейт"),
        ("FPS", "FPS"),
        ("Auto", "Авто"),
        ("Other Default Options", "Інші типові параметри"),
        ("Voice call", "Голосовий виклик"),
        ("Text chat", "Текстовий чат"),
        ("Stop voice call", "Завершити голосовий виклик"),
        ("relay_hint_tip", "Якщо відсутня можливості підключитись напряму, ви можете спробувати підключення через ретранслятор. \nТакож, якщо ви хочете відразу використовувати ретранслятор, можна додати суфікс \"/r\" до ID, або ж вибрати опцію \"Завжди підключатися через ретранслятор\" в картці нещодавніх сеансів."),
        ("Reconnect", "Перепідключитися"),
        ("Codec", "Кодек"),
        ("Resolution", "Роздільна здатність"),
        ("No transfers in progress", "Наразі нічого не пересилається"),
        ("Set one-time password length", "Вказати довжину одноразового пароля"),
        ("RDP Settings", "Налаштування RDP"),
        ("Sort by", "Сортувати за"),
        ("New Connection", "Нове підключення"),
        ("Restore", "Відновити"),
        ("Minimize", "Згорнути"),
        ("Maximize", "Розгорнути"),
        ("Your Device", "Вам пристрій"),
        ("empty_recent_tip", "Овва, відсутні нещодавні сеанси!\nСаме час запланувати нове підключення."),
        ("empty_favorite_tip", "Досі немає улюблених вузлів?\nДавайте організуємо нове підключення та додамо його до улюблених!"),
        ("empty_lan_tip", "О ні, схоже ми поки не виявили жодного віддаленого пристрою"),
        ("empty_address_book_tip", "Ой лишенько, схоже до вашої адресної книги немає жодного віддаленого пристрою"),
        ("eg: admin", "напр., admin"),
        ("Empty Username", "Незаповнене імʼя"),
        ("Empty Password", "Незаповнений пароль"),
        ("Me", "Я"),
        ("identical_file_tip", "Цей файл ідентичний з тим, що на вузлі"),
        ("show_monitors_tip", "Показувати монітори на панелі інструментів"),
        ("View Mode", "Режим перегляду"),
        ("login_linux_tip", "Вам необхідно залогуватися у віддалений акаунт Linux, щоб увімкнути стільничний сеанс X"),
        ("verify_PNDesk_password_tip", "Перевірте пароль PNDesk"),
        ("remember_account_tip", "Запамʼятати цей акаунт"),
        ("os_account_desk_tip", "Цей акаунт використовується для входу до віддаленої ОС та вмикання сеансу стільниці в неграфічному режимі"),
        ("OS Account", "Користувач ОС"),
        ("another_user_login_title_tip", "Інший користувач вже залогований"),
        ("another_user_login_text_tip", "Відʼєднатися"),
        ("xorg_not_found_title_tip", "Xorg не знайдено"),
        ("xorg_not_found_text_tip", "Будь ласка, встановіть Xorg"),
        ("no_desktop_title_tip", "Жодне стільничне середовище не доступне"),
        ("no_desktop_text_tip", "Будь ласка, встановіть стільничне середовище GNOME"),
        ("No need to elevate", "Немає потреби в розширенні прав"),
        ("System Sound", "Системний звук"),
        ("Default", "Типово"),
        ("New RDP", "Нове RDP"),
        ("Fingerprint", "Відбитки пальців"),
        ("Copy Fingerprint", "Копіювати відбитки пальців"),
        ("no fingerprints", "немає відбитків пальців"),
        ("Select a peer", "Оберіть віддалений пристрій"),
        ("Select peers", "Оберіть віддалені пристрої"),
        ("Plugins", "Плагіни"),
        ("Uninstall", "Видалити"),
        ("Update", "Оновити"),
        ("Enable", "Увімкнути"),
        ("Disable", "Вимкнути"),
        ("Options", "Опції"),
        ("resolution_original_tip", "Початкова роздільна здатність"),
        ("resolution_fit_local_tip", "Припасувати поточну роздільну здатність"),
        ("resolution_custom_tip", "Користувацька роздільна здатність"),
        ("Collapse toolbar", "Згорнути панель інструментів"),
        ("Accept and Elevate", "Погодитись та розширити права"),
        ("accept_and_elevate_btn_tooltip", "Погодити підключення та розширити дозволи UAC."),
        ("clipboard_wait_response_timeout_tip", "Вийшов час очікування копіювання."),
        ("Incoming connection", "Вхідне підключення"),
        ("Outgoing connection", "Вихідне підключення"),
        ("Exit", "Вийти"),
        ("Open", "Відкрити"),
        ("logout_tip", "Ви впевнені, що хочете вилогуватися?"),
        ("Service", "Служба"),
        ("Start", "Запустити"),
        ("Stop", "Зупинити"),
        ("exceed_max_devices", "У вас максимальна кількість керованих пристроїв."),
        ("Sync with recent sessions", "Синхронізація з нещодавніми сеансами"),
        ("Sort tags", "Сортувати теги"),
        ("Open connection in new tab", "Відкрити підключення в новій вкладці"),
        ("Move tab to new window", "Перемістити вкладку до нового вікна"),
        ("Can not be empty", "Не може бути порожнім"),
        ("Already exists", "Вже існує"),
        ("Change Password", "Змінити пароль"),
        ("Refresh Password", "Оновити пароль"),
        ("ID", "ID"),
        ("Grid View", "Перегляд ґраткою"),
        ("List View", "Перегляд списком"),
        ("Select", "Вибрати"),
        ("Toggle Tags", "Видимість тегів"),
        ("pull_ab_failed_tip", "Не вдалося оновити адресну книгу"),
        ("push_ab_failed_tip", "Не вдалося синхронізувати адресну книгу"),
        ("synced_peer_readded_tip", "Пристрої з нещодавніх сеансів будуть синхронізовані з адресною книгою"),
        ("Change Color", "Змінити колір"),
        ("Primary Color", "Основний колір"),
        ("HSV Color", "Колір HSV"),
        ("Installation Successful!", "Успішне встановлення!"),
        ("Installation failed!", "Невдале встановлення!"),
        ("Reverse mouse wheel", "Зворотній напрям прокрутки"),
        ("{} sessions", "{} сеансів"),
        ("scam_title", "Вас можуть ОБМАНУТИ!"),
        ("scam_text1", "Якщо ви розмовляєте по телефону з кимось, кого НЕ ЗНАЄТЕ чи кому НЕ ДОВІРЯЄТЕ, і ця особа хоче, щоб ви використали PNDesk та запустили службу, не робіть цього та негайно завершіть дзвінок."),
        ("scam_text2", "Ймовірно, ви маєте справу з шахраєм, що намагається викрасти ваші гроші чи особисті дані."),
        ("Don't show again", "Не показувати знову"),
        ("I Agree", "Я погоджуюсь"),
        ("Decline", "Я не погоджуюсь"),
        ("Timeout in minutes", "Час очікування в хвилинах"),
        ("auto_disconnect_option_tip", "Автоматично завершувати вхідні сеанси в разі неактивності користувача"),
        ("Connection failed due to inactivity", "Зʼєднання розірвано через неактивність"),
        ("Check for software update on startup", "Перевіряти оновлення під час запуску"),
        ("upgrade_PNDesk_server_pro_to_{}_tip", "Будь ласка, оновіть PNDesk Server Pro до версії {} чи більш актуальної!"),
        ("pull_group_failed_tip", "Не вдалося оновити групу"),
        ("Filter by intersection", "Фільтр за збігом"),
        ("Remove wallpaper during incoming sessions", "Прибирати шпалеру під час вхідних сеансів"),
        ("Test", "Тест"),
        ("display_is_plugged_out_msg", "Дисплей відключено, перемкніться на перший дисплей"),
        ("No displays", "Відсутні дисплеї"),
        ("elevated_switch_display_msg", "Перемкніться на основний дисплей, оскільки в режимі розширених прав одночасне використання декілька дисплеїв не підтримуються."),
        ("Open in new window", "Відкрити в новому вікні"),
        ("Show displays as individual windows", "Показувати дисплеї в окремих вікнах"),
        ("Use all my displays for the remote session", "Використовувати всі мої дисплеї для віддаленого сеансу"),
        ("selinux_tip", "SELinux увімкнено на вашому пристрої, що може ускладнити для іншої сторони віддалене керування за допомогою PNDesk."),
        ("Change view", "Режим перегляду"),
        ("Big tiles", "Великі плитки"),
        ("Small tiles", "Маленькі плитки"),
        ("List", "Список"),
        ("Virtual display", "Віртуальний дисплей"),
        ("Plug out all", "Відключити все"),
        ("True color (4:4:4)", "Справжній колір (4:4:4)"),
        ("Enable blocking user input", "Блокувати введення для користувача"),
        ("id_input_tip", "Ви можете ввести ID, безпосередню IP, або ж домен з портом (<домен>:<порт>).\nЯкщо ви хочете отримати доступ до пристрою на іншому сервері, будь ласка, додайте адресу сервера (<id>@<адреса_сервера>?key=<значення_ключа>), наприклад,\n9123456234@192.168.16.1:21117?key=5Qbwsde3unUcJBtrx9ZkvUmwFNoExHzpryHuPUdqlWM=.\nЯкщо ви хочете отримати доступ до пристрою на публічному сервері, будь ласка, введіть \"<id>@public\", ключ для публічного сервера не потрібен."),
        ("privacy_mode_impl_mag_tip", "Режим 1"),
        ("privacy_mode_impl_virtual_display_tip", "Режим 2"),
        ("Enter privacy mode", "Увійти в режим конфіденційності"),
        ("Exit privacy mode", "Вийти з режиму конфіденційності"),
        ("idd_not_support_under_win10_2004_tip", "Драйвер непрямого відображення не підтримується. Потрібна Windows 10 версії 2004 або новіше."),
        ("switch_display_elevated_connections_tip", "В режимі розширених прав, коли є декілька підключень, не підтримується перемикання на неосновний дисплей. Якщо ви хочете керувати декількома дисплеями, будь ласка, спробуйте це після встановлення."),
        ("input_source_1_tip", "Джерело введення 1"),
        ("input_source_2_tip", "Джерело введення 2"),
        ("capture_display_elevated_connections_tip", "В режимі розширених прав захоплення декількох дисплеїв не підтримується. Якщо ви хочете керувати декількома дисплеями, будь ласка, спробуйте це після встановлення."),
        ("Swap control-command key", "Поміняти місцями клавіші Control та Command"),
        ("swap-left-right-mouse", "Поміняти місцями ліву та праву кнопки миші"),
        ("2FA code", "Код двофакторної автентифікації"),
        ("More", "Більше"),
        ("enable-2fa-title", "Увімкнути двофакторну автентифікацію"),
        ("enable-2fa-desc", "Будь ласка, налаштуйте ваш автентифікатор зараз. Ви можете використати програму-автентифікатор, таку як Authy, Microsoft Authenticator або Google Authenticator на телефоні чи компʼютері.\n\nПроскануйте QR-код за допомогою програми та введіть код, який показує програма, щоб увімкнути двофакторну автентифікацію."),
        ("wrong-2fa-code", "Не вдається підтвердити код. Перевірте код та налаштування місцевого часу"),
        ("enter-2fa-title", "Двофакторна автентифікація"),
        ("Email verification code must be 6 characters.", "Код підтвердження з email повинен складатися з 6 символів."),
        ("2FA code must be 6 digits.", "Код двофакторної автентифікації повинен складатися з 6 символів."),
        ("Multiple Windows sessions found", "Виявлено декілька сеансів Windows"),
        ("Please select the session you want to connect to", "Будь ласка, оберіть сеанс, до якого ви хочете підключитися"),
        ("powered_by_me", "На основі PNDesk"),
        ("outgoing_only_desk_tip", "Це персоналізована версія.\nВи можете підключатися до інших пристроїв, але інші пристрої не можуть підключатися до вашого."),
        ("preset_password_warning", "Ця персоналізована версія містить попередньо встановлений пароль. Будь-хто з цим паролем може отримати повний доступ до вашого пристрою. Якщо це неочікувано для вас, негайно видаліть цю програму."),
        ("Security Alert", "Попередження щодо безпеки"),
        ("My address book", "Моя адресна книга"),
        ("Personal", "Особиста"),
        ("Owner", "Власник"),
        ("Set shared password", "Встановити спільний пароль"),
        ("Exist in", "Існує у"),
        ("Read-only", "Лише читання"),
        ("Read/Write", "Читання/запис"),
        ("Full Control", "Повний доступ"),
        ("share_warning_tip", "Поля вище є спільними та видимі для інших."),
        ("Everyone", "Всі"),
        ("ab_web_console_tip", "Детальніше про веб-консоль"),
        ("allow-only-conn-window-open-tip", ""),
        ("no_need_privacy_mode_no_physical_displays_tip", ""),
        ("Follow remote cursor", ""),
        ("Follow remote window focus", ""),
        ("default_proxy_tip", ""),
        ("no_audio_input_device_tip", ""),
        ("Incoming", ""),
        ("Outgoing", ""),
        ("Clear Wayland screen selection", ""),
        ("clear_Wayland_screen_selection_tip", ""),
        ("confirm_clear_Wayland_screen_selection_tip", ""),
        ("android_new_voice_call_tip", ""),
    ].iter().cloned().collect();
}
