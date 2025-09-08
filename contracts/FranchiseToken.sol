// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/security/Pausable.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";

/**
 * @title FranchiseToken
 * @dev ERC20 токен для франшизной сети фудтраков в Грузии
 * 
 * Особенности:
 * - Автоматическая эмиссия при продажах
 * - Распределение токенов между владельцем, франчайзи и покупателем
 * - Управление нодами и POS системами
 * - Роялти для основателя
 */
contract FranchiseToken is ERC20, Ownable, Pausable, ReentrancyGuard {
    
    // Константы токеномики
    uint256 public constant SCALE = 100; // 1 токен = 100 subunits
    uint256 public constant OWNER_OWNER_SHARE = 51; // 0.51 токена владельцу собственной точки
    uint256 public constant OWNER_BUYER_SHARE = 49; // 0.49 токена покупателю
    uint256 public constant FRANCHISE_OWNER_SHARE = 48; // 0.48 токена франчайзи
    uint256 public constant FRANCHISE_ROYALTY_SHARE = 3; // 0.03 токена основателю (роялти)
    uint256 public constant FRANCHISE_BUYER_SHARE = 49; // 0.49 токена покупателю
    
    // Типы нод
    enum NodeType { OWNER, FRANCHISE }
    
    // Структура ноды
    struct Node {
        address owner;
        NodeType nodeType;
        string city;
        bool active;
        uint256 registeredAt;
        uint256 salesCount;
        uint256 totalRevenue;
    }
    
    // Структура продажи
    struct Sale {
        uint256 nodeId;
        string saleId;
        uint256 timestamp;
        uint256 priceGEL;
        address buyer;
        string buyerMeta;
        string posId;
        bool processed;
    }
    
    // Структура эмиссии токенов
    struct TokenMinting {
        uint256 saleId;
        uint256 mintedUnits;
        uint256 ownerUnits;
        uint256 buyerUnits;
        uint256 royaltyUnits;
        uint256 timestamp;
    }
    
    // Переменные состояния
    mapping(uint256 => Node) public nodes;
    mapping(address => bool) public whitelistedPOS;
    mapping(string => address) public checkAddresses;
    mapping(string => Sale) public sales;
    mapping(string => TokenMinting) public tokenMintings;
    
    uint256 public nextNodeId = 1;
    uint256 public totalSales = 0;
    uint256 public totalMinted = 0;
    
    // События
    event NodeRegistered(uint256 indexed nodeId, address indexed owner, NodeType nodeType, string city);
    event SaleRecorded(uint256 indexed nodeId, string indexed saleId, address indexed buyer, uint256 priceGEL);
    event TokensMinted(string indexed saleId, uint256 mintedUnits, uint256 ownerUnits, uint256 buyerUnits, uint256 royaltyUnits);
    event CheckAddressCreated(string indexed saleId, address indexed checkAddress);
    event POSWhitelisted(address indexed posAddress, bool whitelisted);
    
    constructor() ERC20("FranchiseToken", "FRT") {
        // Генезис: создаем 1 токен для основателя
        _mint(msg.sender, SCALE);
        totalMinted = SCALE;
    }
    
    /**
     * @dev Регистрация новой ноды
     * @param nodeOwner Адрес владельца ноды
     * @param nodeType Тип ноды (OWNER или FRANCHISE)
     * @param city Город расположения ноды
     */
    function registerNode(
        address nodeOwner,
        NodeType nodeType,
        string memory city
    ) external onlyOwner returns (uint256) {
        uint256 nodeId = nextNodeId;
        nextNodeId++;
        
        nodes[nodeId] = Node({
            owner: nodeOwner,
            nodeType: nodeType,
            city: city,
            active: true,
            registeredAt: block.timestamp,
            salesCount: 0,
            totalRevenue: 0
        });
        
        emit NodeRegistered(nodeId, nodeOwner, nodeType, city);
        return nodeId;
    }
    
    /**
     * @dev Создание детерминированного чек-адреса
     * @param saleId ID продажи
     */
    function createCheckAddress(string memory saleId) external onlyOwner returns (address) {
        // Генерируем детерминированный адрес на основе saleId
        bytes32 hash = keccak256(abi.encodePacked(saleId, "check_address", block.timestamp));
        address checkAddress = address(uint160(uint256(hash)));
        
        checkAddresses[saleId] = checkAddress;
        emit CheckAddressCreated(saleId, checkAddress);
        
        return checkAddress;
    }
    
    /**
     * @dev Запись продажи и автоматическая эмиссия токенов
     * @param nodeId ID ноды
     * @param saleId ID продажи
     * @param priceGEL Цена в лари
     * @param buyer Адрес покупателя
     * @param buyerMeta Метаданные покупателя
     * @param posId ID POS системы
     */
    function recordSale(
        uint256 nodeId,
        string memory saleId,
        uint256 priceGEL,
        address buyer,
        string memory buyerMeta,
        string memory posId
    ) external onlyWhitelistedPOS whenNotPaused nonReentrant {
        require(nodes[nodeId].active, "Node is not active");
        require(!sales[saleId].processed, "Sale already processed");
        
        // Создаем чек-адрес если его нет
        if (checkAddresses[saleId] == address(0)) {
            createCheckAddress(saleId);
        }
        
        // Записываем продажу
        sales[saleId] = Sale({
            nodeId: nodeId,
            saleId: saleId,
            timestamp: block.timestamp,
            priceGEL: priceGEL,
            buyer: buyer,
            buyerMeta: buyerMeta,
            posId: posId,
            processed: true
        });
        
        // Обновляем статистику ноды
        nodes[nodeId].salesCount++;
        nodes[nodeId].totalRevenue += priceGEL;
        totalSales++;
        
        // Эмитируем и распределяем токены
        _mintAndDistribute(nodeId, buyer, saleId);
        
        emit SaleRecorded(nodeId, saleId, buyer, priceGEL);
    }
    
    /**
     * @dev Внутренняя функция эмиссии и распределения токенов
     */
    function _mintAndDistribute(uint256 nodeId, address buyer, string memory saleId) internal {
        Node memory node = nodes[nodeId];
        uint256 mintedUnits = SCALE; // 1 токен = 100 subunits
        totalMinted += mintedUnits;
        
        uint256 ownerUnits;
        uint256 buyerUnits;
        uint256 royaltyUnits;
        
        if (node.nodeType == NodeType.OWNER) {
            // Собственная точка: 51% владельцу, 49% покупателю
            ownerUnits = OWNER_OWNER_SHARE;
            buyerUnits = OWNER_BUYER_SHARE;
            royaltyUnits = 0;
        } else {
            // Франшиза: 48% франчайзи, 3% роялти, 49% покупателю
            ownerUnits = FRANCHISE_OWNER_SHARE;
            buyerUnits = FRANCHISE_BUYER_SHARE;
            royaltyUnits = FRANCHISE_ROYALTY_SHARE;
        }
        
        // Эмитируем токены
        _mint(node.owner, ownerUnits);
        _mint(buyer, buyerUnits);
        
        if (royaltyUnits > 0) {
            _mint(owner(), royaltyUnits);
        }
        
        // Записываем информацию об эмиссии
        tokenMintings[saleId] = TokenMinting({
            saleId: 0, // Будет заполнено позже
            mintedUnits: mintedUnits,
            ownerUnits: ownerUnits,
            buyerUnits: buyerUnits,
            royaltyUnits: royaltyUnits,
            timestamp: block.timestamp
        });
        
        emit TokensMinted(saleId, mintedUnits, ownerUnits, buyerUnits, royaltyUnits);
    }
    
    /**
     * @dev Добавление POS системы в whitelist
     */
    function whitelistPOS(address posAddress, bool whitelisted) external onlyOwner {
        whitelistedPOS[posAddress] = whitelisted;
        emit POSWhitelisted(posAddress, whitelisted);
    }
    
    /**
     * @dev Получение информации о ноде
     */
    function getNodeInfo(uint256 nodeId) external view returns (
        address owner,
        NodeType nodeType,
        string memory city,
        bool active,
        uint256 registeredAt,
        uint256 salesCount,
        uint256 totalRevenue
    ) {
        Node memory node = nodes[nodeId];
        return (
            node.owner,
            node.nodeType,
            node.city,
            node.active,
            node.registeredAt,
            node.salesCount,
            node.totalRevenue
        );
    }
    
    /**
     * @dev Получение статистики сети
     */
    function getNetworkStats() external view returns (
        uint256 totalNodes,
        uint256 activeNodes,
        uint256 totalSalesCount,
        uint256 totalTokensMinted,
        uint256 masterOwnerBalance
    ) {
        uint256 activeCount = 0;
        for (uint256 i = 1; i < nextNodeId; i++) {
            if (nodes[i].active) {
                activeCount++;
            }
        }
        
        return (
            nextNodeId - 1,
            activeCount,
            totalSales,
            totalMinted,
            balanceOf(owner())
        );
    }
    
    /**
     * @dev Получение баланса кошелька
     */
    function getWalletBalance(address wallet) external view returns (uint256) {
        return balanceOf(wallet);
    }
    
    /**
     * @dev Активация/деактивация ноды
     */
    function setNodeActive(uint256 nodeId, bool active) external onlyOwner {
        nodes[nodeId].active = active;
    }
    
    /**
     * @dev Пауза контракта (экстренная остановка)
     */
    function pause() external onlyOwner {
        _pause();
    }
    
    /**
     * @dev Возобновление работы контракта
     */
    function unpause() external onlyOwner {
        _unpause();
    }
    
    /**
     * @dev Модификатор для проверки whitelisted POS
     */
    modifier onlyWhitelistedPOS() {
        require(whitelistedPOS[msg.sender], "POS not whitelisted");
        _;
    }
}
