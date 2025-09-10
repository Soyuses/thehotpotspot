use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use blockchain_project::franchise_network::{FranchiseNetwork, FranchiseNode, NodeType};
use blockchain_project::hd_wallet::{HDWalletManager, WalletType};
use blockchain_project::kyc_aml::{KYCAmlManager, KYCLevel};
use std::time::SystemTime;

fn bench_franchise_network_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("franchise_network");
    
    for size in [10, 100, 1000, 10000].iter() {
        group.bench_with_input(BenchmarkId::new("add_nodes", size), size, |b, &size| {
            b.iter(|| {
                let mut network = FranchiseNetwork::new();
                for i in 0..size {
                    let node = FranchiseNode {
                        id: i,
                        name: format!("Node_{}", i),
                        node_type: NodeType::Restaurant,
                        location: format!("Location_{}", i),
                        capacity: 100,
                        current_load: 0,
                        is_active: true,
                        created_at: SystemTime::now(),
                        last_updated: SystemTime::now(),
                    };
                    network.add_node(black_box(node));
                }
            });
        });
        
        group.bench_with_input(BenchmarkId::new("get_nodes", size), size, |b, &size| {
            let mut network = FranchiseNetwork::new();
            for i in 0..size {
                let node = FranchiseNode {
                    id: i,
                    name: format!("Node_{}", i),
                    node_type: NodeType::Restaurant,
                    location: format!("Location_{}", i),
                    capacity: 100,
                    current_load: 0,
                    is_active: true,
                    created_at: SystemTime::now(),
                    last_updated: SystemTime::now(),
                };
                network.add_node(node);
            }
            
            b.iter(|| {
                for i in 0..size {
                    black_box(network.get_node(i));
                }
            });
        });
    }
    
    group.finish();
}

fn bench_wallet_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("wallet_operations");
    
    for count in [10, 100, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("generate_wallets", count), count, |b, &count| {
            b.iter(|| {
                let mut wallet_manager = HDWalletManager::new();
                let seed_phrase = "test seed phrase for benchmarking";
                
                for i in 0..count {
                    let wallet_type = if i % 2 == 0 { WalletType::Node } else { WalletType::Check };
                    black_box(wallet_manager.generate_wallet(seed_phrase, wallet_type));
                }
            });
        });
    }
    
    group.finish();
}

fn bench_kyc_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("kyc_operations");
    
    for count in [10, 100, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("register_users", count), count, |b, &count| {
            b.iter(|| {
                let mut kyc_manager = KYCAmlManager::new();
                
                for i in 0..count {
                    let user_id = format!("user_{}", i);
                    black_box(kyc_manager.register_user(
                        &user_id,
                        &format!("user{}@example.com", i),
                        &format!("FirstName{}", i),
                        &format!("LastName{}", i),
                        Some("GE".to_string())
                    ));
                }
            });
        });
        
        group.bench_with_input(BenchmarkId::new("start_kyc_process", count), count, |b, &count| {
            let mut kyc_manager = KYCAmlManager::new();
            
            // Предварительно регистрируем пользователей
            for i in 0..count {
                let user_id = format!("user_{}", i);
                kyc_manager.register_user(
                    &user_id,
                    &format!("user{}@example.com", i),
                    &format!("FirstName{}", i),
                    &format!("LastName{}", i),
                    Some("GE".to_string())
                ).unwrap();
            }
            
            b.iter(|| {
                for i in 0..count {
                    let user_id = format!("user_{}", i);
                    black_box(kyc_manager.start_kyc_process(&user_id, KYCLevel::Basic));
                }
            });
        });
    }
    
    group.finish();
}

fn bench_concurrent_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrent_operations");
    
    group.bench_function("concurrent_wallet_generation", |b| {
        b.iter(|| {
            use std::sync::Arc;
            use std::thread;
            
            let wallet_manager = Arc::new(HDWalletManager::new());
            let mut handles = vec![];
            
            for i in 0..10 {
                let manager = Arc::clone(&wallet_manager);
                let handle = thread::spawn(move || {
                    let seed_phrase = format!("test seed phrase {}", i);
                    for j in 0..100 {
                        let wallet_type = if j % 2 == 0 { WalletType::Node } else { WalletType::Check };
                        black_box(manager.generate_wallet(&seed_phrase, wallet_type));
                    }
                });
                handles.push(handle);
            }
            
            for handle in handles {
                handle.join().unwrap();
            }
        });
    });
    
    group.finish();
}

fn bench_memory_usage(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_usage");
    
    group.bench_function("large_network_memory", |b| {
        b.iter(|| {
            let mut network = FranchiseNetwork::new();
            
            // Создаем большую сеть
            for i in 0..10000 {
                let node = FranchiseNode {
                    id: i,
                    name: format!("Node_{}", i),
                    node_type: NodeType::Restaurant,
                    location: format!("Location_{}", i),
                    capacity: 100,
                    current_load: 0,
                    is_active: true,
                    created_at: SystemTime::now(),
                    last_updated: SystemTime::now(),
                };
                network.add_node(node);
            }
            
            // Проверяем, что все узлы доступны
            for i in 0..10000 {
                black_box(network.get_node(i));
            }
        });
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_franchise_network_operations,
    bench_wallet_operations,
    bench_kyc_operations,
    bench_concurrent_operations,
    bench_memory_usage
);

criterion_main!(benches);
