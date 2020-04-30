use crate::Octocrab;

/// Filter by current status of the pull request.
#[derive(Debug, Clone, Copy, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PullRequestState {
    All,
    Open,
    Closed,
}

/// What to sort results by. Can be either `created`, `updated`, `popularity`
/// (comment count) or `long-running` (age, filtering by pulls updated in the
/// last month).
#[derive(Debug, Clone, Copy, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PullRequestSorting {
    Created,
    Updated,
    Popularity,
    LongRunning,
}

/// What to sort results by. Can be either `created`, `updated`, `popularity`
/// (comment count) or `long-running` (age, filtering by pulls updated in the
/// last month).
#[derive(Debug, Clone, Copy, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PullRequestDirection {
    Ascending,
    Descending,
}

pub struct PullRequestHandler<'octo> {
    crab: &'octo Octocrab,
    owner: String,
    repo: String,
}

impl<'octo> PullRequestHandler<'octo> {
    pub fn new(crab: &'octo Octocrab, owner: String, repo: String) -> Self {
        Self { crab, owner, repo }
    }

    /// Checks if a given pull request has been merged.
    pub async fn is_merged(&self, pr: u64) -> crate::Result<bool> {
        let url = format!(
            "/repos/{owner}/{repo}/pulls/{pr}/merge",
            owner = self.owner,
            repo = self.repo,
            pr = pr
        );
        let response = self.crab._get(url, None::<&()>).await?;

        Ok(response.status() == 204)
    }

    /// Get's a given pull request with by its `pr` number.
    pub async fn get(&self, pr: u64) -> crate::Result<crate::models::PullRequest> {
        let url = format!(
            "/repos/{owner}/{repo}/pulls/{pr}",
            owner = self.owner,
            repo = self.repo,
            pr = pr
        );
        self.crab.get(url, None::<&()>).await
    }

    /// Get's a given pull request with by its `pr` number.
    pub async fn create(
        &self,
        title: impl Into<String>,
        head: impl Into<String>,
        base: impl Into<String>,
    ) -> CreatePullRequestBuilder<'octo, '_> {
        CreatePullRequestBuilder::new(self, title, head, base)
    }

    /// Creates a new `ListPullRequestsBuilder` that can be configured to filter
    /// listing pulling requests.
    pub fn list(&self) -> ListPullRequestsBuilder {
        ListPullRequestsBuilder::new(self)
    }
}

#[derive(serde::Serialize)]
pub struct CreatePullRequestBuilder<'octo, 'b> {
    #[serde(skip)]
    handler: &'b PullRequestHandler<'octo>,
    title: String,
    head: String,
    base: String,
    body: Option<String>,
    draft: Option<bool>,
    maintainer_can_modify: Option<bool>,
}

impl<'octo, 'b> CreatePullRequestBuilder<'octo, 'b> {
    pub fn new(
        handler: &'b PullRequestHandler<'octo>,
        title: impl Into<String>,
        head: impl Into<String>,
        base: impl Into<String>,
    ) -> Self {
        Self {
            handler,
            title: title.into(),
            head: head.into(),
            base: base.into(),
            body: None,
            draft: None,
            maintainer_can_modify: None,
        }
    }

    /// Set the body of the pull request
    pub fn body(mut self, body: impl Into<Option<String>>) -> Self {
        self.body = body.into();
        self
    }

    /// Set the pull request as a draft.
    pub fn draft(mut self, draft: impl Into<Option<bool>>) -> Self {
        self.draft = draft.into();
        self
    }

    /// Set whether other maintainers can modify the pull request.
    pub fn maintainer_can_modify(mut self, maintainer_can_modify: impl Into<Option<bool>>) -> Self {
        self.maintainer_can_modify = maintainer_can_modify.into();
        self
    }

    pub async fn send(self) -> crate::Result<crate::models::PullRequest> {
        let url = format!(
            "/repos/{owner}/{repo}/pulls",
            owner = self.handler.owner,
            repo = self.handler.repo
        );

        self.handler.crab.post(url, Some(&self)).await
    }
}

#[derive(serde::Serialize)]
pub struct ListPullRequestsBuilder<'octo, 'b> {
    #[serde(skip)]
    handler: &'b PullRequestHandler<'octo>,
    state: Option<PullRequestState>,
    head: Option<String>,
    base: Option<String>,
    sort: Option<PullRequestSorting>,
    direction: Option<PullRequestDirection>,
    per_page: Option<u8>,
    page: Option<usize>,
}

impl<'octo, 'b> ListPullRequestsBuilder<'octo, 'b> {
    fn new(handler: &'b PullRequestHandler<'octo>) -> Self {
        Self {
            handler,
            state: None,
            head: None,
            base: None,
            sort: None,
            direction: None,
            per_page: None,
            page: None,
        }
    }

    /// Filter pull requests by `PullRequestState`.
    pub fn state(mut self, state: PullRequestState) -> Self {
        self.state = Some(state);
        self
    }

    /// Filter pull requests by head user or head organization and branch name
    /// in the format of `user:ref-name` or `organization:ref-name`. For
    /// example: `github:new-script-format` or `octocrab:test-branch`.
    pub fn head(mut self, head: impl Into<String>) -> Self {
        self.head = Some(head.into());
        self
    }

    /// Filter pulls by base branch name. Example: `gh-pages`.
    pub fn base(mut self, base: impl Into<String>) -> Self {
        self.base = Some(base.into());
        self
    }

    /// What to sort results by. Can be either `created`, `updated`,
    /// `popularity` (comment count) or `long-running` (age, filtering by pulls
    /// updated in the last month).
    pub fn sort(mut self, sort: impl Into<PullRequestSorting>) -> Self {
        self.sort = Some(sort.into());
        self
    }

    /// The direction of the sort. Can be either ascending or descending.
    /// Default: descending when sort is `created` or sort is not specified,
    /// otherwise ascending sort.
    pub fn direction(mut self, direction: impl Into<PullRequestDirection>) -> Self {
        self.direction = Some(direction.into());
        self
    }

    /// Results per page (max 100).
    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    /// Page number of the results to fetch.
    pub fn page(mut self, page: impl Into<usize>) -> Self {
        self.page = Some(page.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<Vec<crate::models::PullRequest>> {
        let url = format!(
            "/repos/{owner}/{repo}/pulls",
            owner = self.handler.owner,
            repo = self.handler.repo
        );
        self.handler.crab.get(url, Some(&self)).await
    }
}