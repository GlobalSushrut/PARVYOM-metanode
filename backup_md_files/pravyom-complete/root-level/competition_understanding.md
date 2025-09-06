# Jigsaw Agile Community Rules Classification - Competition Understanding

## Competition Overview

The **Jigsaw Agile Community Rules Classification** is a Kaggle competition focused on building AI models to help Reddit moderators automatically detect rule violations in community posts. This competition is part of Jigsaw's ongoing efforts to improve online community moderation using machine learning.

### Key Objectives
- Build a binary classification model to predict whether a Reddit comment violates specific subreddit rules
- Help moderators uphold community-specific norms more efficiently
- Advance AI-assisted content moderation capabilities

## Dataset Structure

### Training Data (`train.csv`)
- **Size**: 2,029 samples
- **Features**: 9 columns
- **Target**: Binary classification (0 = No violation, 1 = Rule violation)

#### Column Descriptions:
1. **`row_id`**: Unique identifier for each sample
2. **`body`**: The Reddit comment text to be classified
3. **`rule`**: The specific subreddit rule being evaluated
4. **`subreddit`**: The subreddit where the comment was posted
5. **`positive_example_1`**: Example of content that violates the rule
6. **`positive_example_2`**: Another example of rule-violating content
7. **`negative_example_1`**: Example of content that does NOT violate the rule
8. **`negative_example_2`**: Another example of non-violating content
9. **`rule_violation`**: Target variable (0 or 1)

### Test Data (`test.csv`)
- **Size**: 10 samples
- **Features**: 8 columns (same as training, excluding `rule_violation`)
- **Purpose**: Final evaluation set for predictions

### Sample Submission (`sample_submission.csv`)
- **Format**: `row_id`, `rule_violation` (probability between 0 and 1)
- **Default prediction**: 0.5 for all samples

## Data Analysis Insights

### Target Distribution
- **Class Balance**: Nearly balanced dataset
  - No violation (0): 998 samples (49.2%)
  - Rule violation (1): 1,031 samples (50.8%)

### Text Characteristics
- **Average comment length**: 177 characters
- **Length range**: 51-499 characters
- **Distribution**: Right-skewed with median at 138 characters

### Rule Categories
The dataset focuses on **two main rule types**:

1. **"No legal advice: Do not offer or request legal advice."** (1,017 samples)
2. **"No Advertising: Spam, referral links, unsolicited advertising, and promotional content are not allowed."** (1,012 samples)

### Subreddit Distribution
- **Total subreddits**: 100 different communities
- **Top subreddits**:
  - `legaladvice`: 213 samples
  - `AskReddit`: 152 samples
  - `soccerstreams`: 139 samples
  - `personalfinance`: 125 samples
  - `relationships`: 106 samples

## Problem Complexity

### Challenge Aspects
1. **Context Dependency**: Rule violations depend heavily on subreddit-specific context
2. **Nuanced Language**: Distinguishing between legitimate advice and rule violations
3. **Few-Shot Learning**: Limited examples per rule-subreddit combination
4. **Spam Detection**: Identifying promotional content vs. legitimate sharing

### Key Features for Classification
1. **Primary Text**: The comment body content
2. **Rule Context**: The specific rule being evaluated
3. **Community Context**: The subreddit where the comment appears
4. **Example Guidance**: Positive and negative examples provide rule interpretation

## Technical Approach Considerations

### Model Architecture Options
1. **Traditional ML**: TF-IDF + Logistic Regression/SVM
2. **Deep Learning**: BERT/RoBERTa-based transformers
3. **Multi-input Models**: Combining text, rule, and subreddit features
4. **Few-shot Learning**: Leveraging example-based learning

### Feature Engineering Opportunities
1. **Text Features**: Length, special characters, URLs, profanity
2. **Rule-specific Features**: Keywords related to legal advice or advertising
3. **Subreddit Features**: Community-specific patterns
4. **Example Similarity**: Semantic similarity to provided examples

### Evaluation Considerations
- **Metric**: Likely binary classification accuracy or AUC-ROC
- **Cross-validation**: Stratified by subreddit and rule type
- **Generalization**: Model should work across different communities

## Business Impact

### Moderation Efficiency
- Reduce manual review workload for moderators
- Enable faster response to rule violations
- Maintain community standards at scale

### Community Health
- Consistent rule enforcement across large communities
- Reduced exposure to spam and inappropriate content
- Better user experience through cleaner discussions

## Competition Strategy

### Data Preprocessing
1. Text cleaning and normalization
2. Handling special characters and URLs
3. Rule and subreddit encoding
4. Example text integration

### Model Development
1. Baseline models with simple features
2. Transformer-based models for text understanding
3. Multi-task learning incorporating examples
4. Ensemble methods for robustness

### Validation Strategy
1. Stratified cross-validation by subreddit
2. Rule-specific performance analysis
3. Error analysis on misclassified samples
4. Generalization testing across communities

## Key Success Factors

1. **Understanding Context**: Properly incorporating subreddit and rule context
2. **Example Utilization**: Effectively using positive/negative examples
3. **Text Representation**: Capturing semantic meaning beyond keywords
4. **Generalization**: Building models that work across diverse communities
5. **Robustness**: Handling edge cases and adversarial examples

## Dataset Quality Notes

- **No Missing Values**: Complete dataset with all fields populated
- **Balanced Classes**: Good balance between violation/non-violation cases
- **Diverse Content**: Wide range of subreddits and comment types
- **Rich Context**: Multiple examples provide clear rule interpretation

This competition represents a practical application of NLP for content moderation, requiring models that can understand nuanced community rules and apply them consistently across diverse online discussions.
