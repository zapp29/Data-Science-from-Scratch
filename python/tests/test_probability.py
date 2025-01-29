from probability import inverse_normal_cdf, normal_cdf, normal_pdf, uniform_cdf, uniform_pdf

def test_uniform_pdf():
    assert uniform_pdf(0.5) == 1
    assert uniform_pdf(1.5) == 0

def test_uniform_cdf():
    assert uniform_cdf(0.5) == 0.5
    assert uniform_cdf(1.5) == 1

def test_normal_pdf():
    assert round(normal_pdf(0), 10) == round(0.3989422804014327, 10)
    assert round(normal_pdf(1), 10) == round(0.24197072451914337, 10)

def test_normal_cdf():
    assert round(normal_cdf(0), 10) == 0.5
    assert round(normal_cdf(1), 10) == round(0.8413447460685429, 10)

def test_inverse_normal_cdf():
    assert round(inverse_normal_cdf(0.5), 2) == 0
    assert round(inverse_normal_cdf(0.8413447460685429), 2) == 1


