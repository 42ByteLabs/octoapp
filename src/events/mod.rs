//! # GitHub Webhook Payloads
//!
//! This module contains the various payloads that GitHub sends to the webhook

pub mod payloads;

/// A wrapper around a webhook payload.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WebHook<T>(pub T);

impl<T> WebHook<T> {
    /// Consumes the wrapper and returns the inner payload.
    ///
    /// # Example
    /// ```rust
    /// # use octoapp::WebHook;
    /// let string = "Hello, world!".to_string();
    /// let webhook = WebHook(string);
    /// let inner = webhook.into_inner();
    /// assert_eq!(inner, "Hello, world!");
    /// ```
    #[inline(always)]
    pub fn into_inner(self) -> T {
        self.0
    }
}

/// Webhook Event Enum
///
/// This enum represents the various events that GitHub sends to the webhook
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum Event {
    /// Branch Protection Rule Event
    BranchProtectionRule(payloads::BranchProtectionRuleEvent),
    /// Check Run Event
    CheckRun(payloads::CheckRunEvent),
    /// Check Suite Event
    CheckSuite(payloads::CheckSuiteEvent),
    /// Code Scanning Alert Event
    CodeScanningAlert(payloads::CodeScanningAlertEvent),
    /// Commit Comment Event
    CommitComment(payloads::CommitCommentEvent),
    /// Content Reference Event
    Create(payloads::CreateEvent),
    /// Delete Event
    Delete(payloads::DeleteEvent),
    /// Dependabot Alert Event
    DependabotAlert(payloads::DependabotAlertEvent),
    /// Deployment Key Event
    DeployKey(payloads::DeployKeyEvent),
    /// Deployment Protection Rule Event
    DeploymentProtectionRule(payloads::DeploymentProtectionRuleEvent),
    /// Deployment Status Event
    DeploymentStatus(payloads::DeploymentStatusEvent),
    /// Deployment Event
    Deployment(payloads::DeploymentEvent),
    /// Discussion Comment Event
    DiscussionComment(payloads::DiscussionCommentEvent),
    /// Discussion Event
    Discussion(payloads::DiscussionEvent),
    /// Fork Event
    Fork(payloads::ForkEvent),
    /// GitHub App Authorization Event
    GithubAppAuthorization(payloads::GithubAppAuthorizationEvent),
    /// Golllum Event
    Gollum(payloads::GollumEvent),
    /// Installation Event
    InstallationRepositories(payloads::InstallationRepositoriesEvent),
    /// Installation Target Event
    InstallationTarget(payloads::InstallationTargetEvent),
    /// Installation Event
    Installation(payloads::InstallationEvent),
    /// Issue Comment Event
    IssueComment(payloads::IssueCommentEvent),
    /// Issues Event
    Issues(payloads::IssuesEvent),
    /// Label Event
    Label(payloads::LabelEvent),
    /// Marketplace Purchase Event
    MarketplacePurchase(payloads::MarketplacePurchaseEvent),
    /// Member Event
    Member(payloads::MemberEvent),
    /// Membership Event
    Membership(payloads::MembershipEvent),
    /// Merge Group Event
    MergeGroup(payloads::MergeGroupEvent),
    /// Meta Event
    Meta(payloads::MetaEvent),
    /// Milestone Event
    Milestone(payloads::MilestoneEvent),
    /// Organization Block Event
    OrgBlock(payloads::OrgBlockEvent),
    /// Organization Event
    Organization(payloads::OrganizationEvent),
    /// Package Event
    Package(payloads::PackageEvent),
    /// Page Build Event
    PageBuild(payloads::PageBuildEvent),
    /// Personal Access Token Event
    PersonalAccessTokenRequest(payloads::PersonalAccessTokenRequestEvent),
    /// Ping Event (used for testing)
    Ping(payloads::PingEvent),
    /// Project Card Event
    ProjectCard(payloads::ProjectCardEvent),
    /// Project Column Event
    ProjectColumn(payloads::ProjectColumnEvent),
    /// Project V2 Item Event
    ProjectsV2Item(payloads::ProjectsV2ItemEvent),
    /// Project V2 Event
    ProjectsV2(payloads::ProjectsV2Event),
    /// Public Event
    Public(payloads::PublicEvent),
    /// Pull Request Review Comment Event
    PullRequestReviewComment(payloads::PullRequestReviewCommentEvent),
    /// Pull Request Review Thread Event
    PullRequestReviewThread(payloads::PullRequestReviewThreadEvent),
    /// Pull Request Review Event
    PullRequestReview(payloads::PullRequestReviewEvent),
    /// Pull Request Event
    PullRequest(payloads::PullRequestEvent),
    /// Push Event
    Push(payloads::PushEvent),
    /// Registry Package Event
    RegistryPackage(payloads::RegistryPackageEvent),
    /// Release Event
    Release(payloads::ReleaseEvent),
    /// Repository Dispatch Event
    RepositoryAdvisory(payloads::RepositoryAdvisoryEvent),
    /// Repository Dispatch Event
    RepositoryImport(payloads::RepositoryImportEvent),
    /// Repository Vulnerability Alert Event
    RepositoryVulnerabilityAlert(payloads::RepositoryVulnerabilityAlertEvent),
    /// Repository Event
    Repository(payloads::RepositoryEvent),
    /// Secret Scanning Alert Location Event
    SecretScanningAlertLocation(payloads::SecretScanningAlertLocationEvent),
    /// Secret Scanning Alert Event
    SecretScanningAlert(payloads::SecretScanningAlertEvent),
    /// Security Advisory Event
    SecurityAdvisory(payloads::SecurityAdvisoryEvent),
    /// Security And Analysis Event
    SecurityAndAnalysis(payloads::SecurityAndAnalysisEvent),
    /// Sponsorship Event
    Sponsorship(payloads::SponsorshipEvent),
    /// Star Event
    Star(payloads::StarEvent),
    /// Status Event
    Status(payloads::StatusEvent),
    /// Team Add Event
    TeamAdd(payloads::TeamAddEvent),
    /// Team Event
    Team(payloads::TeamEvent),
    /// Watch Event
    Watch(payloads::WatchEvent),
    /// Workflow Dispatch Event
    WorkflowDispatch(payloads::WorkflowDispatchEvent),
    /// Workflow Job Event
    WorkflowJob(payloads::WorkflowJobEvent),
}
