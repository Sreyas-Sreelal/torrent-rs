#[repr(usize)]
pub enum Categories{
	Error = 1,
	Peer = 2,
	PortMapping = 4,
	Storage = 8,
	Tracker = 16,
	Debug = 32,
	Status = 64,
	Progress = 128,
	IpBlock = 256,
	PerformanceWarning = 512,
	Dht = 1024,
	All = 4294967295,
}