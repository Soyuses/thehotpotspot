// Main page component
import { apiClient } from '@/services/api';
import { formatCurrency } from '@/utils/format';

export class MainPage {
  private container: HTMLElement;

  constructor(container: HTMLElement) {
    this.container = container;
  }

  async render(): Promise<void> {
    this.container.innerHTML = `
      <div class="main-page">
        <header class="hero">
          <div class="hero-content">
            <h1>🍲 The Hot Pot Spot</h1>
            <p class="hero-subtitle">Революционная ресторанная сеть с блокчейн-технологиями</p>
            <div class="hero-stats">
              <div class="stat">
                <span class="stat-number" id="totalOrders">-</span>
                <span class="stat-label">Заказов</span>
              </div>
              <div class="stat">
                <span class="stat-number" id="totalTokens">-</span>
                <span class="stat-label">Токенов выдано</span>
              </div>
              <div class="stat">
                <span class="stat-number" id="activeUsers">-</span>
                <span class="stat-label">Активных пользователей</span>
              </div>
            </div>
          </div>
        </header>

        <section class="features">
          <h2>Наши возможности</h2>
          <div class="features-grid">
            <div class="feature-card">
              <div class="feature-icon">🔗</div>
              <h3>Блокчейн-технологии</h3>
              <p>Прозрачные транзакции и безопасные платежи с использованием криптовалют</p>
            </div>
            <div class="feature-card">
              <div class="feature-icon">🎥</div>
              <h3>Видеонаблюдение</h3>
              <p>Прямая трансляция из кухни с анонимизацией лиц для вашей конфиденциальности</p>
            </div>
            <div class="feature-card">
              <div class="feature-icon">🪙</div>
              <h3>Токеномика</h3>
              <p>Зарабатывайте SPOT токены за активность и участвуйте в управлении сетью</p>
            </div>
            <div class="feature-card">
              <div class="feature-icon">🏪</div>
              <h3>Франшизная сеть</h3>
              <p>Развивающаяся сеть ресторанов с единой системой управления</p>
            </div>
          </div>
        </section>

        <section class="interfaces">
          <h2>Панели управления</h2>
          <div class="interfaces-grid">
            <a href="owner.html" class="interface-card owner-card">
              <h3>👑 Панель владельца</h3>
              <p>Управление сетью, токеномика, мониторинг</p>
            </a>
            <a href="franchise.html" class="interface-card franchise-card">
              <h3>🏪 Панель франчайзи</h3>
              <p>Управление рестораном, заказы, статистика</p>
            </a>
            <a href="transparency.html" class="interface-card transparency-card">
              <h3>🔍 Панель прозрачности</h3>
              <p>Отчеты, аудит, репутация</p>
            </a>
            <a href="video.html" class="interface-card video-card">
              <h3>📹 Видеонаблюдение</h3>
              <p>Управление видеопотоками, мониторинг кухни</p>
            </a>
            <a href="tablet.html" class="interface-card tablet-card">
              <h3>📱 Планшет за столом</h3>
              <p>Заказ еды, стриминг, информация о SPOT</p>
            </a>
          </div>
        </section>

        <section class="menu-preview">
          <h2>Наше меню</h2>
          <div class="menu-grid" id="menuPreview">
            <div class="loading">Загрузка меню...</div>
          </div>
          <a href="franchise.html" class="btn btn-primary">Посмотреть полное меню</a>
        </section>

        <footer class="footer">
          <div class="footer-content">
            <p>&copy; 2024 The Hot Pot Spot. Все права защищены.</p>
            <div class="footer-links">
              <a href="#privacy">Конфиденциальность</a>
              <a href="#terms">Условия использования</a>
              <a href="#contact">Контакты</a>
            </div>
          </div>
        </footer>
      </div>
    `;

    await this.loadData();
    this.attachEventListeners();
  }

  private async loadData(): Promise<void> {
    try {
      // Load menu preview
      const menu = await apiClient.getMenu();
      this.renderMenuPreview(menu.slice(0, 6)); // Show first 6 items

      // Load statistics (mock data for now)
      this.updateStatistics({
        totalOrders: 1250,
        totalTokens: 45000,
        activeUsers: 320
      });
    } catch (error) {
      console.error('Error loading main page data:', error);
      this.showError('Ошибка загрузки данных');
    }
  }

  private renderMenuPreview(menuItems: any[]): void {
    const menuContainer = document.getElementById('menuPreview');
    if (!menuContainer) return;

    if (menuItems.length === 0) {
      menuContainer.innerHTML = '<div class="no-data">Меню временно недоступно</div>';
      return;
    }

    menuContainer.innerHTML = menuItems.map(item => `
      <div class="menu-item">
        <div class="menu-item-image">
          <div class="placeholder-image">🍽️</div>
        </div>
        <div class="menu-item-content">
          <h4>${item.name}</h4>
          <p>${item.description}</p>
          <div class="menu-item-price">${formatCurrency(item.price_subunits / 100)}</div>
        </div>
      </div>
    `).join('');
  }

  private updateStatistics(stats: { totalOrders: number; totalTokens: number; activeUsers: number }): void {
    const totalOrdersEl = document.getElementById('totalOrders');
    const totalTokensEl = document.getElementById('totalTokens');
    const activeUsersEl = document.getElementById('activeUsers');

    if (totalOrdersEl) totalOrdersEl.textContent = stats.totalOrders.toLocaleString();
    if (totalTokensEl) totalTokensEl.textContent = stats.totalTokens.toLocaleString();
    if (activeUsersEl) activeUsersEl.textContent = stats.activeUsers.toLocaleString();
  }

  private attachEventListeners(): void {
    // Add any event listeners here
  }

  private showError(message: string): void {
    const errorDiv = document.createElement('div');
    errorDiv.className = 'error-message';
    errorDiv.textContent = message;
    this.container.appendChild(errorDiv);
  }
}
