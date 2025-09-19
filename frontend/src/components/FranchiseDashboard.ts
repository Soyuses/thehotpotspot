// Franchise Dashboard component
import { apiClient } from '@/services/api';
import { formatCurrency, formatDate, formatOrderStatus } from '@/utils/format';
import type { MenuItem, Order, CreateOrderRequest } from '@/types/api';

export class FranchiseDashboard {
  private container: HTMLElement;
  private currentTab: string = 'orders';
  private menu: MenuItem[] = [];
  private orders: Order[] = [];

  constructor(container: HTMLElement) {
    this.container = container;
  }

  async render(): Promise<void> {
    this.container.innerHTML = `
      <div class="franchise-dashboard">
        <header class="dashboard-header">
          <h1>🏪 Панель франчайзи</h1>
          <div class="header-actions">
            <button class="btn btn-secondary" id="refreshBtn">🔄 Обновить</button>
            <button class="btn btn-primary" id="addItemBtn">➕ Добавить блюдо</button>
          </div>
        </header>

        <nav class="tabs">
          <button class="tab active" data-tab="orders">Заказы</button>
          <button class="tab" data-tab="menu">Меню</button>
          <button class="tab" data-tab="statistics">Статистика</button>
          <button class="tab" data-tab="settings">Настройки</button>
        </nav>

        <main class="tab-content">
          <div id="orders" class="tab-panel active">
            ${this.renderOrdersTab()}
          </div>
          <div id="menu" class="tab-panel">
            ${this.renderMenuTab()}
          </div>
          <div id="statistics" class="tab-panel">
            ${this.renderStatisticsTab()}
          </div>
          <div id="settings" class="tab-panel">
            ${this.renderSettingsTab()}
          </div>
        </main>
      </div>
    `;

    await this.loadData();
    this.attachEventListeners();
  }

  private renderOrdersTab(): string {
    return `
      <div class="orders">
        <div class="orders-header">
          <h2>Управление заказами</h2>
          <div class="order-filters">
            <select id="statusFilter">
              <option value="">Все статусы</option>
              <option value="pending">Ожидает</option>
              <option value="confirmed">Подтвержден</option>
              <option value="cooking">Готовится</option>
              <option value="ready">Готов</option>
              <option value="delivered">Доставлен</option>
              <option value="cancelled">Отменен</option>
            </select>
          </div>
        </div>
        
        <div class="orders-list" id="ordersList">
          <div class="loading">Загрузка заказов...</div>
        </div>
      </div>
    `;
  }

  private renderMenuTab(): string {
    return `
      <div class="menu-management">
        <div class="menu-header">
          <h2>Управление меню</h2>
          <button class="btn btn-primary" id="addMenuItemBtn">➕ Добавить блюдо</button>
        </div>
        
        <div class="menu-list" id="menuList">
          <div class="loading">Загрузка меню...</div>
        </div>
      </div>
    `;
  }

  private renderStatisticsTab(): string {
    return `
      <div class="statistics">
        <div class="stats-grid">
          <div class="stat-card">
            <h3>Заказы сегодня</h3>
            <div class="stat-value" id="todayOrders">0</div>
          </div>
          <div class="stat-card">
            <h3>Выручка сегодня</h3>
            <div class="stat-value" id="todayRevenue">0.00 GEL</div>
          </div>
          <div class="stat-card">
            <h3>Средний чек</h3>
            <div class="stat-value" id="averageOrder">0.00 GEL</div>
          </div>
          <div class="stat-card">
            <h3>Активные заказы</h3>
            <div class="stat-value" id="activeOrders">0</div>
          </div>
        </div>
        
        <div class="chart-container">
          <h3>График заказов</h3>
          <canvas id="ordersChart" width="400" height="200"></canvas>
        </div>
      </div>
    `;
  }

  private renderSettingsTab(): string {
    return `
      <div class="settings">
        <div class="settings-section">
          <h3>Настройки ресторана</h3>
          <form id="restaurantSettings">
            <div class="form-group">
              <label for="restaurantName">Название ресторана</label>
              <input type="text" id="restaurantName" value="The Hot Pot Spot - Тбилиси" />
            </div>
            <div class="form-group">
              <label for="restaurantAddress">Адрес</label>
              <input type="text" id="restaurantAddress" value="ул. Руставели, 1, Тбилиси" />
            </div>
            <div class="form-group">
              <label for="restaurantPhone">Телефон</label>
              <input type="tel" id="restaurantPhone" value="+995 32 123 4567" />
            </div>
            <div class="form-group">
              <label for="workingHours">Часы работы</label>
              <input type="text" id="workingHours" value="10:00 - 22:00" />
            </div>
            <button type="submit" class="btn btn-primary">Сохранить настройки</button>
          </form>
        </div>
        
        <div class="settings-section">
          <h3>Настройки уведомлений</h3>
          <div class="checkbox-group">
            <label>
              <input type="checkbox" id="newOrderNotifications" checked />
              Уведомления о новых заказах
            </label>
            <label>
              <input type="checkbox" id="orderStatusNotifications" checked />
              Уведомления об изменении статуса заказа
            </label>
            <label>
              <input type="checkbox" id="lowStockNotifications" checked />
              Уведомления о низком запасе ингредиентов
            </label>
          </div>
        </div>
      </div>
    `;
  }

  private async loadData(): Promise<void> {
    try {
      // Load menu
      this.menu = await apiClient.getMenu();
      this.renderMenu();

      // Load orders (mock data for now)
      this.orders = await this.loadMockOrders();
      this.renderOrders();

      // Load statistics
      await this.loadStatistics();
    } catch (error) {
      console.error('Error loading franchise dashboard data:', error);
      this.showError('Ошибка загрузки данных');
    }
  }

  private async loadMockOrders(): Promise<Order[]> {
    // Mock orders data
    return [
      {
        id: 'ORD-001',
        customer_wallet: '0x1234...5678',
        items: [
          { menu_item_id: '1', quantity: 2, price_subunits: 1500 },
          { menu_item_id: '2', quantity: 1, price_subunits: 800 }
        ],
        total_amount: 3800,
        delivery_time_minutes: 25,
        status: 'pending',
        created_at: new Date().toISOString()
      },
      {
        id: 'ORD-002',
        customer_wallet: '0x9876...5432',
        items: [
          { menu_item_id: '3', quantity: 1, price_subunits: 1200 }
        ],
        total_amount: 1200,
        delivery_time_minutes: 15,
        status: 'cooking',
        created_at: new Date(Date.now() - 300000).toISOString()
      }
    ];
  }

  private renderOrders(): void {
    const container = document.getElementById('ordersList');
    if (!container) return;

    if (this.orders.length === 0) {
      container.innerHTML = '<div class="no-data">Нет заказов</div>';
      return;
    }

    container.innerHTML = this.orders.map(order => `
      <div class="order-card" data-order-id="${order.id}">
        <div class="order-header">
          <div class="order-id">Заказ #${order.id}</div>
          <div class="order-status status-${order.status}">
            ${formatOrderStatus(order.status)}
          </div>
        </div>
        <div class="order-details">
          <div class="order-customer">
            <strong>Клиент:</strong> ${order.customer_wallet}
          </div>
          <div class="order-items">
            <strong>Блюда:</strong>
            <ul>
              ${order.items.map(item => `
                <li>${this.getMenuItemName(item.menu_item_id)} x${item.quantity} - ${formatCurrency(item.price_subunits / 100)}</li>
              `).join('')}
            </ul>
          </div>
          <div class="order-total">
            <strong>Итого: ${formatCurrency(order.total_amount / 100)}</strong>
          </div>
          <div class="order-time">
            <strong>Время доставки:</strong> ${order.delivery_time_minutes} мин.
          </div>
          <div class="order-created">
            <strong>Создан:</strong> ${formatDate(order.created_at)}
          </div>
        </div>
        <div class="order-actions">
          ${this.getOrderActions(order)}
        </div>
      </div>
    `).join('');
  }

  private getMenuItemName(menuItemId: string): string {
    const item = this.menu.find(m => m.id === menuItemId);
    return item ? item.name : `Блюдо #${menuItemId}`;
  }

  private getOrderActions(order: Order): string {
    const actions = [];
    
    if (order.status === 'pending') {
      actions.push(`<button class="btn btn-success" onclick="franchiseApp.confirmOrder('${order.id}')">✓ Подтвердить</button>`);
      actions.push(`<button class="btn btn-danger" onclick="franchiseApp.cancelOrder('${order.id}')">✗ Отменить</button>`);
    } else if (order.status === 'confirmed') {
      actions.push(`<button class="btn btn-warning" onclick="franchiseApp.startCooking('${order.id}')">🍳 Начать готовку</button>`);
    } else if (order.status === 'cooking') {
      actions.push(`<button class="btn btn-primary" onclick="franchiseApp.finishCooking('${order.id}')">✅ Готово</button>`);
    } else if (order.status === 'ready') {
      actions.push(`<button class="btn btn-success" onclick="franchiseApp.deliverOrder('${order.id}')">🚚 Доставлено</button>`);
    }
    
    return actions.join(' ');
  }

  private renderMenu(): void {
    const container = document.getElementById('menuList');
    if (!container) return;

    if (this.menu.length === 0) {
      container.innerHTML = '<div class="no-data">Меню пусто</div>';
      return;
    }

    container.innerHTML = this.menu.map(item => `
      <div class="menu-item-card" data-item-id="${item.id}">
        <div class="menu-item-image">
          <div class="placeholder-image">🍽️</div>
        </div>
        <div class="menu-item-content">
          <h4>${item.name}</h4>
          <p>${item.description}</p>
          <div class="menu-item-details">
            <span class="price">${formatCurrency(item.price_subunits / 100)}</span>
            <span class="cooking-time">${item.cooking_time_minutes} мин.</span>
            <span class="availability">В наличии: ${item.availability}</span>
          </div>
        </div>
        <div class="menu-item-actions">
          <button class="btn btn-sm btn-secondary" onclick="franchiseApp.editMenuItem('${item.id}')">✏️</button>
          <button class="btn btn-sm btn-danger" onclick="franchiseApp.deleteMenuItem('${item.id}')">🗑️</button>
        </div>
      </div>
    `).join('');
  }

  private async loadStatistics(): Promise<void> {
    // Mock statistics data
    const stats = {
      todayOrders: 15,
      todayRevenue: 450.50,
      averageOrder: 30.03,
      activeOrders: 3
    };

    document.getElementById('todayOrders')!.textContent = stats.todayOrders.toString();
    document.getElementById('todayRevenue')!.textContent = formatCurrency(stats.todayRevenue);
    document.getElementById('averageOrder')!.textContent = formatCurrency(stats.averageOrder);
    document.getElementById('activeOrders')!.textContent = stats.activeOrders.toString();
  }

  private attachEventListeners(): void {
    // Tab switching
    document.querySelectorAll('.tab').forEach(tab => {
      tab.addEventListener('click', (e) => {
        const target = e.target as HTMLElement;
        const tabName = target.dataset.tab;
        if (tabName) {
          this.switchTab(tabName);
        }
      });
    });

    // Status filter
    document.getElementById('statusFilter')?.addEventListener('change', (e) => {
      const target = e.target as HTMLSelectElement;
      this.filterOrdersByStatus(target.value);
    });

    // Refresh button
    document.getElementById('refreshBtn')?.addEventListener('click', () => {
      this.loadData();
    });

    // Add item button
    document.getElementById('addItemBtn')?.addEventListener('click', () => {
      this.showAddItemModal();
    });

    // Settings form
    document.getElementById('restaurantSettings')?.addEventListener('submit', (e) => {
      e.preventDefault();
      this.saveSettings();
    });
  }

  private switchTab(tabName: string): void {
    // Update tab buttons
    document.querySelectorAll('.tab').forEach(tab => {
      tab.classList.remove('active');
    });
    document.querySelector(`[data-tab="${tabName}"]`)?.classList.add('active');

    // Update tab panels
    document.querySelectorAll('.tab-panel').forEach(panel => {
      panel.classList.remove('active');
    });
    document.getElementById(tabName)?.classList.add('active');

    this.currentTab = tabName;
  }

  private filterOrdersByStatus(status: string): void {
    const orderCards = document.querySelectorAll('.order-card');
    
    orderCards.forEach(card => {
      const orderStatus = card.querySelector('.order-status')?.textContent?.toLowerCase();
      if (!status || orderStatus === status) {
        (card as HTMLElement).style.display = 'block';
      } else {
        (card as HTMLElement).style.display = 'none';
      }
    });
  }

  private showAddItemModal(): void {
    // Implement add item modal
    alert('Функция добавления блюда будет реализована');
  }

  private saveSettings(): void {
    // Implement settings save
    alert('Настройки сохранены');
  }

  // Public methods for order actions
  public async confirmOrder(orderId: string): Promise<void> {
    try {
      const success = await apiClient.confirmOrder(orderId);
      if (success) {
        this.showSuccess('Заказ подтвержден');
        await this.loadData();
      } else {
        this.showError('Ошибка подтверждения заказа');
      }
    } catch (error) {
      this.showError('Ошибка подтверждения заказа');
    }
  }

  public async cancelOrder(orderId: string): Promise<void> {
    const reason = prompt('Причина отмены заказа:');
    if (!reason) return;

    try {
      const success = await apiClient.cancelOrder(orderId, reason, 'franchise_wallet');
      if (success) {
        this.showSuccess('Заказ отменен');
        await this.loadData();
      } else {
        this.showError('Ошибка отмены заказа');
      }
    } catch (error) {
      this.showError('Ошибка отмены заказа');
    }
  }

  public startCooking(orderId: string): void {
    // Implement start cooking
    this.showSuccess('Готовка начата');
  }

  public finishCooking(orderId: string): void {
    // Implement finish cooking
    this.showSuccess('Блюдо готово');
  }

  public deliverOrder(orderId: string): void {
    // Implement deliver order
    this.showSuccess('Заказ доставлен');
  }

  public editMenuItem(itemId: string): void {
    // Implement edit menu item
    alert('Редактирование блюда будет реализовано');
  }

  public deleteMenuItem(itemId: string): void {
    if (confirm('Удалить это блюдо из меню?')) {
      // Implement delete menu item
      this.showSuccess('Блюдо удалено');
    }
  }

  private showError(message: string): void {
    console.error(message);
    alert(message); // Replace with proper notification component
  }

  private showSuccess(message: string): void {
    console.log(message);
    alert(message); // Replace with proper notification component
  }
}
