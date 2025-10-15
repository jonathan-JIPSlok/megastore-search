use crate::product::Product;

/// Retorna uma lista com vários produtos pré-carregados.
pub fn load_products() -> Vec<Product> {
    vec![
        
        // Smartphones
        Product::new(1, "iPhone 15", "Smartphone", "Apple"),
        Product::new(2, "Galaxy S24", "Smartphone", "Samsung"),
        Product::new(3, "Redmi Note 13", "Smartphone", "Xiaomi"),
        Product::new(4, "Moto G84", "Smartphone", "Motorola"),
        Product::new(5, "Realme 11 Pro", "Smartphone", "Realme"),
        Product::new(6, "Asus Zenfone 10", "Smartphone", "Asus"),
        Product::new(7, "Nokia G400", "Smartphone", "Nokia"),
        Product::new(8, "Pixel 8", "Smartphone", "Google"),
        Product::new(9, "Honor Magic 6", "Smartphone", "Honor"),
        Product::new(10, "Sony Xperia 10 V", "Smartphone", "Sony"),

        // Notebooks
        Product::new(11, "MacBook Air M2", "Notebook", "Apple"),
        Product::new(12, "MacBook Pro M3", "Notebook", "Apple"),
        Product::new(13, "Notebook Aspire 5", "Notebook", "Acer"),
        Product::new(14, "Notebook Ideapad 3i", "Notebook", "Lenovo"),
        Product::new(15, "Dell Inspiron 15", "Notebook", "Dell"),
        Product::new(16, "Asus Vivobook 16", "Notebook", "Asus"),
        Product::new(17, "HP Pavilion 14", "Notebook", "HP"),
        Product::new(18, "Samsung Book3", "Notebook", "Samsung"),
        Product::new(19, "Avell A70 HYB", "Notebook", "Avell"),
        Product::new(20, "MSI Katana GF66", "Notebook", "MSI"),

        // Monitores
        Product::new(21, "Monitor LG UltraWide 29\"", "Monitor", "LG"),
        Product::new(22, "Monitor Samsung Odyssey G5", "Monitor", "Samsung"),
        Product::new(23, "Monitor Dell P2422H", "Monitor", "Dell"),
        Product::new(24, "Monitor Acer Nitro 24\"", "Monitor", "Acer"),
        Product::new(25, "Monitor AOC Hero 27\"", "Monitor", "AOC"),
        Product::new(26, "Monitor Philips 246E9QDSB", "Monitor", "Philips"),
        Product::new(27, "Monitor Gigabyte G27F", "Monitor", "Gigabyte"),
        Product::new(28, "Monitor Asus TUF VG259Q", "Monitor", "Asus"),
        Product::new(29, "Monitor BenQ Zowie XL2546K", "Monitor", "BenQ"),
        Product::new(30, "Monitor LG 32UN880-B Ergo", "Monitor", "LG"),

        // Armazenamento
        Product::new(31, "SSD NVMe 1TB", "Armazenamento", "Kingston"),
        Product::new(32, "SSD SATA 480GB", "Armazenamento", "Crucial"),
        Product::new(33, "HD Externo 2TB", "Armazenamento", "Seagate"),
        Product::new(34, "HD Interno 1TB", "Armazenamento", "Western Digital"),
        Product::new(35, "SSD Externo 500GB", "Armazenamento", "Samsung"),
        Product::new(36, "Pendrive 64GB", "Armazenamento", "Sandisk"),
        Product::new(37, "Cartão SD 128GB", "Armazenamento", "Kingston"),
        Product::new(38, "SSD NVMe 2TB", "Armazenamento", "Corsair"),
        Product::new(39, "HD Externo 4TB", "Armazenamento", "Toshiba"),
        Product::new(40, "SSD Kingston Fury 1TB", "Armazenamento", "Kingston"),

        // Periféricos
        Product::new(41, "Logitech MX Master 3", "Mouse", "Logitech"),
        Product::new(42, "Razer DeathAdder V3", "Mouse", "Razer"),
        Product::new(43, "Corsair Harpoon Wireless", "Mouse", "Corsair"),
        Product::new(44, "Redragon Cobra M711", "Mouse", "Redragon"),
        Product::new(45, "HyperX Pulsefire Haste", "Mouse", "HyperX"),
        Product::new(46, "Razer BlackWidow V3", "Teclado", "Razer"),
        Product::new(47, "Logitech G Pro X Keyboard", "Teclado", "Logitech"),
        Product::new(48, "Corsair K70 RGB", "Teclado", "Corsair"),
        Product::new(49, "Redragon Kumara", "Teclado", "Redragon"),
        Product::new(50, "HyperX Alloy Origins", "Teclado", "HyperX"),

        // Fones
        Product::new(51, "AirPods Pro 2", "Fone de Ouvido", "Apple"),
        Product::new(52, "Galaxy Buds 3", "Fone de Ouvido", "Samsung"),
        Product::new(53, "Sony WH-1000XM5", "Fone de Ouvido", "Sony"),
        Product::new(54, "JBL Tune 760NC", "Fone de Ouvido", "JBL"),
        Product::new(55, "Beats Studio Pro", "Fone de Ouvido", "Beats"),
        Product::new(56, "Logitech Zone Vibe 100", "Fone de Ouvido", "Logitech"),
        Product::new(57, "HyperX Cloud Alpha", "Fone de Ouvido", "HyperX"),
        Product::new(58, "Razer Kraken X", "Fone de Ouvido", "Razer"),
        Product::new(59, "Edifier W820NB", "Fone de Ouvido", "Edifier"),
        Product::new(60, "Philips TAH8506", "Fone de Ouvido", "Philips"),

        // Consoles e jogos
        Product::new(61, "PlayStation 5", "Console", "Sony"),
        Product::new(62, "Xbox Series X", "Console", "Microsoft"),
        Product::new(63, "Nintendo Switch OLED", "Console", "Nintendo"),
        Product::new(64, "Steam Deck", "Console", "Valve"),
        Product::new(65, "PlayStation Portal", "Console", "Sony"),
        Product::new(66, "Controle DualSense", "Acessório", "Sony"),
        Product::new(67, "Controle Xbox Elite", "Acessório", "Microsoft"),
        Product::new(68, "Nintendo Pro Controller", "Acessório", "Nintendo"),
        Product::new(69, "Headset Pulse 3D", "Acessório", "Sony"),
        Product::new(70, "PlayStation VR2", "Console", "Sony"),

        // Casa Inteligente
        Product::new(71, "Echo Dot 5ª Geração", "Assistente Virtual", "Amazon"),
        Product::new(72, "Google Nest Mini", "Assistente Virtual", "Google"),
        Product::new(73, "Lâmpada Inteligente Positivo", "Casa Inteligente", "Positivo"),
        Product::new(74, "Câmera Intelbras iM4", "Segurança", "Intelbras"),
        Product::new(75, "Tomada Smart Wi-Fi", "Casa Inteligente", "Philips"),
        Product::new(76, "Robô Aspirador WAP", "Casa Inteligente", "WAP"),
        Product::new(77, "Fechadura Digital Intelbras", "Segurança", "Intelbras"),
        Product::new(78, "Interruptor Wi-Fi Sonoff", "Casa Inteligente", "Sonoff"),
        Product::new(79, "Smart TV 55\"", "Televisão", "LG"),
        Product::new(80, "Smart TV 65\" QLED", "Televisão", "Samsung"),

        // Impressoras e periféricos
        Product::new(81, "Impressora EcoTank L3250", "Impressora", "Epson"),
        Product::new(82, "Impressora HP Ink Tank 416", "Impressora", "HP"),
        Product::new(83, "Canon G3110", "Impressora", "Canon"),
        Product::new(84, "Brother DCP-1602", "Impressora", "Brother"),
        Product::new(85, "Scanner Canon Lide 300", "Scanner", "Canon"),
        Product::new(86, "Plotter Epson T3170", "Impressora", "Epson"),
        Product::new(87, "Toner HP 12A", "Acessório", "HP"),
        Product::new(88, "Cartucho Canon CL-141", "Acessório", "Canon"),
        Product::new(89, "Tinta Epson 544", "Acessório", "Epson"),
        Product::new(90, "Papel Fotográfico Glossy A4", "Acessório", "Chamex"),

        // Mobiliário e acessórios
        Product::new(91, "Cadeira Gamer ThunderX3", "Mobiliário", "ThunderX3"),
        Product::new(92, "Cadeira de Escritório Ergonômica", "Mobiliário", "Flexform"),
        Product::new(93, "Mesa para Computador Office", "Mobiliário", "Madesa"),
        Product::new(94, "Suporte para Monitor Ajustável", "Acessório", "ELG"),
        Product::new(95, "Mousepad Gamer XXL", "Acessório", "Redragon"),
        Product::new(96, "Base Refrigerada para Notebook", "Acessório", "Cooler Master"),
        Product::new(97, "Organizador de Cabos", "Acessório", "Multilaser"),
        Product::new(98, "Suporte de Notebook Articulado", "Acessório", "C3Tech"),
        Product::new(99, "Luminária de Mesa LED", "Acessório", "Positivo"),
        Product::new(100, "Fone de Mesa com Microfone", "Acessório", "Intelbras"),

        // Livros e eletrônicos variados
        Product::new(101, "Kindle Paperwhite", "Leitor Digital", "Amazon"),
        Product::new(102, "Roteador TP-Link AX1800", "Rede", "TP-Link"),
        Product::new(103, "Repetidor de Sinal TP-Link", "Rede", "TP-Link"),
        Product::new(104, "Switch 8 Portas Gigabit", "Rede", "Intelbras"),
        Product::new(105, "Adaptador Bluetooth 5.0", "Rede", "Orico"),
        Product::new(106, "Webcam Logitech C920", "Periférico", "Logitech"),
        Product::new(107, "Microfone Fifine K690", "Periférico", "Fifine"),
        Product::new(108, "Caixa de Som JBL Flip 6", "Áudio", "JBL"),
        Product::new(109, "Caixa de Som Anker Soundcore", "Áudio", "Anker"),
        Product::new(110, "Projetor Epson EpiqVision", "Projeção", "Epson"),
        Product::new(111, "Controle Universal Inteligente", "Casa Inteligente", "Broadlink"),
        Product::new(112, "Estabilizador SMS Progressive", "Energia", "SMS"),
        Product::new(113, "Nobreak APC 1500VA", "Energia", "APC"),
        Product::new(114, "Carregador Anker Nano 30W", "Acessório", "Anker"),
        Product::new(115, "Cabo USB-C 2m", "Acessório", "Ugreen"),
        Product::new(116, "Cabo HDMI 4K 3m", "Acessório", "Belkin"),
        Product::new(117, "Hub USB 4 Portas", "Acessório", "Orico"),
        Product::new(118, "Suporte para Celular de Mesa", "Acessório", "Baseus"),
        Product::new(119, "Tripé Fotográfico 1,6m", "Acessório", "Yunteng"),
        Product::new(120, "Leitor de Cartão SD USB", "Acessório", "Kingston"),
    ]
}
