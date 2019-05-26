#[repr(C)]
#[derive(Copy, Clone)]
pub enum StateType{
	QueuedForChecking,
	CheckingFiles,
	DownloadingMetadata,
	Downloading,
	Finished,
	Seeding,
	Allocating,
	CheckingResumeData,
}