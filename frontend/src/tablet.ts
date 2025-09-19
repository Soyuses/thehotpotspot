// Tablet Dashboard entry point
import { apiClient } from './services/api';
import { formatCurrency } from './utils/format';

class TabletApp {
  private container: HTMLElement;
  private cart: Array<{ item: any; quantity: number }> = [];

  constructor() {
    this.container = document.getElementById('app')!;
    this.init();
  }

  private async init(): Promise<void> {
    try {
      // Check API health
      const isHealthy = await apiClient.healthCheck();
      if (!isHealthy) {
        this.showError('API сервер недоступен. Проверьте подключение.');
        return;
      }

      // Initialize tablet dashboard
      await this.render();

      console.log('Tablet dashboard initialized successfully');
    } catch (error) {
      console.error('Failed to initialize tablet app:', error);
      this.showError('Ошибка инициализации планшета');
    }
  }

  private async render(): Promise<void> {
    this.container.innerHTML = `
      <div class="tablet-dashboard">
        <header class="tablet-header">
          <h1>🍲 The Hot Pot Spot</h1>
          <div class="tablet-info">
            <span class="table-number">Стол #1</span>
            <span class="spot-info" id="spotInfo">ℹ️ О SPOT токенах</span>
          </div>
        </header>

        <main class="tablet-content">
          <div class="tablet-grid">
            <div class="menu-section">
              <h2>Наше меню</h2>
              <div class="menu-categories">
                <button class="category-btn active" data-category="all">Все</button>
                <button class="category-btn" data-category="hotpot">Хот-пот</button>
                <button class="category-btn" data-category="appetizers">Закуски</button>
                <button class="category-btn" data-category="drinks">Напитки</button>
              </div>
              <div class="menu-items" id="menuItems">
                <div class="loading">Загрузка меню...</div>
              </div>
            </div>

            <div class="cart-section">
              <h2>Ваш заказ</h2>
              <div class="cart-items" id="cartItems">
                <p class="empty-cart">Корзина пуста</p>
              </div>
              <div class="cart-total" id="cartTotal">
                <div class="total-amount">Итого: 0.00 GEL</div>
                <button class="btn btn-primary btn-large" id="placeOrderBtn" disabled>
                  🛒 Оформить заказ
                </button>
              </div>
            </div>
          </div>

          <div class="streaming-section">
            <h2>Прямая трансляция из кухни</h2>
            <div class="video-container">
              <div class="video-placeholder">
                <div class="video-icon">📹</div>
                <p>Прямая трансляция из кухни</p>
                <div class="streaming-info">
                  <span class="stream-status">🔴 В эфире</span>
                  <span class="stream-time">45:00</span>
                </div>
              </div>
            </div>
          </div>

          <div class="spot-info-modal" id="spotInfoModal">
            <div class="modal-content">
              <h3>🪙 Что такое SPOT токены?</h3>
              <div class="spot-explanation">
                <p><strong>SPOT</strong> - это utility-токены нашей сети ресторанов.</p>
                <h4>Как заработать SPOT:</h4>
                <ul>
                  <li>1 SPOT за каждое 5-е посещение</li>
                  <li>1 SPOT за 2 часа просмотра стримов</li>
                  <li>1 SPOT за репост или комментарий с 50+ лайками</li>
                  <li>1 SPOT за сессию стриминга (до 45 минут)</li>
                </ul>
                <h4>Что с ними делать:</h4>
                <p>Держатели SPOT с балансом выше медианного участвуют в распределении 50% неиспользованных чеков в конце года!</p>
              </div>
              <button class="btn btn-primary" onclick="tabletApp.closeSpotInfo()">
                Понятно
              </button>
            </div>
          </div>
        </main>
      </div>
    `;

    await this.loadData();
    this.attachEventListeners();
  }

  private async loadData(): Promise<void> {
    try {
      const menu = await apiClient.getMenu();
      this.renderMenu(menu);
    } catch (error) {
      console.error('Error loading tablet data:', error);
      this.showError('Ошибка загрузки меню');
    }
  }

  private renderMenu(menu: any[]): void {
    const container = document.getElementById('menuItems');
    if (!container) return;

    if (menu.length === 0) {
      container.innerHTML = '<div class="no-data">Меню временно недоступно</div>';
      return;
    }

    container.innerHTML = menu.map(item => `
      <div class="menu-item" data-item-id="${item.id}">
        <div class="item-image">
          <div class="placeholder-image">🍽️</div>
        </div>
        <div class="item-content">
          <h4>${item.name}</h4>
          <p>${item.description}</p>
          <div class="item-details">
            <span class="price">${formatCurrency(item.price_subunits / 100)}</span>
            <span class="cooking-time">${item.cooking_time_minutes} мин.</span>
          </div>
        </div>
        <div class="item-actions">
          <button class="btn btn-sm btn-secondary" onclick="tabletApp.decreaseQuantity('${item.id}')">-</button>
          <span class="quantity" id="qty_${item.id}">0</span>
          <button class="btn btn-sm btn-primary" onclick="tabletApp.increaseQuantity('${item.id}')">+</button>
        </div>
      </div>
    `).join('');
  }

  private renderCart(): void {
    const container = document.getElementById('cartItems');
    const totalContainer = document.getElementById('cartTotal');
    const placeOrderBtn = document.getElementById('placeOrderBtn');

    if (!container || !totalContainer || !placeOrderBtn) return;

    if (this.cart.length === 0) {
      container.innerHTML = '<p class="empty-cart">Корзина пуста</p>';
      totalContainer.innerHTML = `
        <div class="total-amount">Итого: 0.00 GEL</div>
        <button class="btn btn-primary btn-large" disabled>🛒 Оформить заказ</button>
      `;
      return;
    }

    let total = 0;
    container.innerHTML = this.cart.map(cartItem => {
      const itemTotal = cartItem.item.price_subunits * cartItem.quantity;
      total += itemTotal;
      
      return `
        <div class="cart-item">
          <div class="cart-item-info">
            <h4>${cartItem.item.name}</h4>
            <span class="cart-item-price">${formatCurrency(itemTotal / 100)}</span>
          </div>
          <div class="cart-item-actions">
            <button class="btn btn-sm btn-secondary" onclick="tabletApp.decreaseQuantity('${cartItem.item.id}')">-</button>
            <span class="quantity">${cartItem.quantity}</span>
            <button class="btn btn-sm btn-primary" onclick="tabletApp.increaseQuantity('${cartItem.item.id}')">+</button>
            <button class="btn btn-sm btn-danger" onclick="tabletApp.removeFromCart('${cartItem.item.id}')">🗑️</button>
          </div>
        </div>
      `;
    }).join('');

    totalContainer.innerHTML = `
      <div class="total-amount">Итого: ${formatCurrency(total / 100)}</div>
      <button class="btn btn-primary btn-large" onclick="tabletApp.placeOrder()">
        🛒 Оформить заказ
      </button>
    `;
  }

  private attachEventListeners(): void {
    // Category filtering
    document.querySelectorAll('.category-btn').forEach(btn => {
      btn.addEventListener('click', (e) => {
        const target = e.target as HTMLElement;
        const category = target.dataset.category;
        
        // Update active button
        document.querySelectorAll('.category-btn').forEach(b => b.classList.remove('active'));
        target.classList.add('active');
        
        // Filter menu items
        this.filterMenuByCategory(category || 'all');
      });
    });

    // SPOT info modal
    document.getElementById('spotInfo')?.addEventListener('click', () => {
      this.showSpotInfo();
    });

    // Close modal on outside click
    document.getElementById('spotInfoModal')?.addEventListener('click', (e) => {
      if (e.target === e.currentTarget) {
        this.closeSpotInfo();
      }
    });
  }

  private filterMenuByCategory(category: string): void {
    const menuItems = document.querySelectorAll('.menu-item');
    
    menuItems.forEach(item => {
      if (category === 'all') {
        (item as HTMLElement).style.display = 'block';
      } else {
        // For now, show all items regardless of category
        (item as HTMLElement).style.display = 'block';
      }
    });
  }

  public increaseQuantity(itemId: string): void {
    const existingItem = this.cart.find(cartItem => cartItem.item.id === itemId);
    
    if (existingItem) {
      existingItem.quantity++;
    } else {
      // Find the menu item
      const menuItem = document.querySelector(`[data-item-id="${itemId}"]`);
      if (menuItem) {
        const name = menuItem.querySelector('h4')?.textContent || '';
        const description = menuItem.querySelector('p')?.textContent || '';
        const priceElement = menuItem.querySelector('.price');
        const price = priceElement ? parseFloat(priceElement.textContent?.replace(/[^\d.]/g, '') || '0') * 100 : 0;
        
        this.cart.push({
          item: {
            id: itemId,
            name,
            description,
            price_subunits: price
          },
          quantity: 1
        });
      }
    }
    
    this.updateQuantityDisplay(itemId);
    this.renderCart();
  }

  public decreaseQuantity(itemId: string): void {
    const existingItem = this.cart.find(cartItem => cartItem.item.id === itemId);
    
    if (existingItem) {
      existingItem.quantity--;
      if (existingItem.quantity <= 0) {
        this.cart = this.cart.filter(cartItem => cartItem.item.id !== itemId);
      }
    }
    
    this.updateQuantityDisplay(itemId);
    this.renderCart();
  }

  public removeFromCart(itemId: string): void {
    this.cart = this.cart.filter(cartItem => cartItem.item.id !== itemId);
    this.updateQuantityDisplay(itemId);
    this.renderCart();
  }

  private updateQuantityDisplay(itemId: string): void {
    const quantityElement = document.getElementById(`qty_${itemId}`);
    if (quantityElement) {
      const cartItem = this.cart.find(cartItem => cartItem.item.id === itemId);
      quantityElement.textContent = cartItem ? cartItem.quantity.toString() : '0';
    }
  }

  public async placeOrder(): Promise<void> {
    if (this.cart.length === 0) {
      this.showError('Корзина пуста');
      return;
    }

    const customerWallet = prompt('Введите адрес вашего кошелька:');
    if (!customerWallet) return;

    try {
      const order = await apiClient.createOrder({
        customer_wallet: customerWallet,
        items: this.cart.map(cartItem => ({
          menu_item_id: cartItem.item.id,
          quantity: cartItem.quantity,
          price_subunits: cartItem.item.price_subunits
        })),
        delivery_time_minutes: 30
      });

      if (order) {
        this.showSuccess('Заказ оформлен успешно!');
        this.cart = [];
        this.renderCart();
        // Reset all quantity displays
        document.querySelectorAll('.quantity').forEach(el => {
          if (el.id.startsWith('qty_')) {
            el.textContent = '0';
          }
        });
      } else {
        this.showError('Ошибка оформления заказа');
      }
    } catch (error) {
      this.showError('Ошибка оформления заказа');
    }
  }

  public showSpotInfo(): void {
    const modal = document.getElementById('spotInfoModal');
    if (modal) {
      modal.style.display = 'flex';
    }
  }

  public closeSpotInfo(): void {
    const modal = document.getElementById('spotInfoModal');
    if (modal) {
      modal.style.display = 'none';
    }
  }

  private showError(message: string): void {
    console.error(message);
    alert(message);
  }

  private showSuccess(message: string): void {
    console.log(message);
    alert(message);
  }
}

// Initialize app when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
  const app = new TabletApp();
  // Make app globally available
  (window as any).tabletApp = app;
});
